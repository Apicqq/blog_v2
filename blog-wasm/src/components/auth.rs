//! Компонент регистрации и входа.

use crate::api;
use crate::storage;
use dioxus::prelude::*;

/// Панель регистрации и входа пользователя.
#[component]
pub(crate) fn AuthPanel(token: Signal<Option<String>>) -> Element {
    let message = use_signal(|| None::<String>);
    let error = use_signal(|| None::<String>);

    rsx! {
        article { class: "auth-panel",
            h2 { "Аккаунт" }

            RegisterForm { token, message, error }
            LoginForm { token, message, error }
            LogoutButton { token, message, error }

            if let Some(current_message) = message.read().as_ref() {
                p { class: "success-message", "{current_message}" }
            }

            if let Some(current_error) = error.read().as_ref() {
                p { class: "error-message", "{current_error}" }
            }
        }
    }
}

#[component]
fn RegisterForm(
    mut token: Signal<Option<String>>,
    mut message: Signal<Option<String>>,
    mut error: Signal<Option<String>>,
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
                                token.set(Some(response.token));
                                error.set(None);
                                message.set(Some(format!(
                                    "Зарегистрирован пользователь {} ({})",
                                    response.user.username, response.user.email
                                )));
                            }
                            Err(api_error) => {
                                message.set(None);
                                error.set(Some(api_error.user_message()));
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
    mut message: Signal<Option<String>>,
    mut error: Signal<Option<String>>,
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
                                token.set(Some(response.token));
                                error.set(None);
                                message.set(Some(format!(
                                    "Выполнен вход: {} ({})",
                                    response.user.username, response.user.email
                                )));
                            }
                            Err(api_error) => {
                                message.set(None);
                                error.set(Some(api_error.user_message()));
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
    mut message: Signal<Option<String>>,
    mut error: Signal<Option<String>>,
) -> Element {
    rsx! {
        if token.read().is_some() {
            button {
                class: "secondary",
                onclick: move |_| {
                    storage::clear_token();
                    token.set(None);
                    error.set(None);
                    message.set(Some("Выполнен выход".to_string()));
                },
                "Logout"
            }
        }
    }
}
