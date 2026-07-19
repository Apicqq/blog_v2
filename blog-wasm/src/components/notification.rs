//! Компонент всплывающих уведомлений.

use dioxus::prelude::*;

/// Тип всплывающего уведомления.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NotificationKind {
    Success,
    Error,
}

impl NotificationKind {
    fn class_name(self) -> &'static str {
        match self {
            Self::Success => "notification notification-success",
            Self::Error => "notification notification-error",
        }
    }
}

/// Состояние всплывающего уведомления.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NotificationState {
    kind: NotificationKind,
    message: String,
}

impl NotificationState {
    /// Создает уведомление об успешном действии.
    pub(crate) fn success(message: impl Into<String>) -> Self {
        Self {
            kind: NotificationKind::Success,
            message: message.into(),
        }
    }

    /// Создает уведомление об ошибке.
    pub(crate) fn error(message: impl Into<String>) -> Self {
        Self {
            kind: NotificationKind::Error,
            message: message.into(),
        }
    }
}

/// Контейнер всплывающего уведомления.
#[component]
pub(crate) fn NotificationHost(mut notification: Signal<Option<NotificationState>>) -> Element {
    let current_notification = notification.read().clone();

    rsx! {
        if let Some(current_notification) = current_notification {
            div { class: "notification-host",
                div {
                    class: current_notification.kind.class_name(),
                    role: "status",
                    span { "{current_notification.message}" }
                    button {
                        class: "notification-close",
                        aria_label: "Закрыть уведомление",
                        onclick: move |_| notification.set(None),
                        "×"
                    }
                }
            }
        }
    }
}
