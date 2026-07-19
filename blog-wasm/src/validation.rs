//! Клиентская валидация пользовательского ввода.

const MIN_USERNAME_LENGTH: usize = 3;
const MIN_PASSWORD_LENGTH: usize = 8;
const MIN_POST_TITLE_LENGTH: usize = 3;

/// Проверяет форму регистрации.
#[must_use]
pub(crate) fn validate_registration(username: &str, email: &str, password: &str) -> Option<String> {
    validate_username(username)
        .or_else(|| validate_email(email))
        .or_else(|| validate_password(password))
}

/// Проверяет форму входа.
#[must_use]
pub(crate) fn validate_login(username: &str, password: &str) -> Option<String> {
    validate_username(username).or_else(|| validate_password(password))
}

/// Проверяет форму поста.
#[must_use]
pub(crate) fn validate_post(title: &str, content: &str) -> Option<String> {
    validate_post_title(title).or_else(|| validate_post_content(content))
}

fn validate_username(username: &str) -> Option<String> {
    let username = username.trim();

    if username.is_empty() {
        return Some("Имя пользователя не должно быть пустым.".to_string());
    }

    if username.chars().count() < MIN_USERNAME_LENGTH {
        return Some("Имя пользователя должно содержать минимум 3 символа.".to_string());
    }

    None
}

fn validate_email(email: &str) -> Option<String> {
    let email = email.trim();

    if email.is_empty() {
        return Some("Почта не должна быть пустой.".to_string());
    }

    let Some((local_part, domain)) = email.split_once('@') else {
        return Some("Введите корректную почту.".to_string());
    };

    if local_part.is_empty() || !domain.contains('.') {
        return Some("Введите корректную почту.".to_string());
    }

    None
}

fn validate_password(password: &str) -> Option<String> {
    if password.is_empty() {
        return Some("Пароль не должен быть пустым.".to_string());
    }

    if password.chars().count() < MIN_PASSWORD_LENGTH {
        return Some("Пароль должен содержать минимум 8 символов.".to_string());
    }

    None
}

fn validate_post_title(title: &str) -> Option<String> {
    let title = title.trim();

    if title.is_empty() {
        return Some("Заголовок не должен быть пустым.".to_string());
    }

    if title.chars().count() < MIN_POST_TITLE_LENGTH {
        return Some("Заголовок должен содержать минимум 3 символа.".to_string());
    }

    None
}

fn validate_post_content(content: &str) -> Option<String> {
    if content.trim().is_empty() {
        return Some("Текст поста не должен быть пустым.".to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_registration_rejects_empty_username() {
        assert_eq!(
            validate_registration("", "alice@example.com", "password"),
            Some("Имя пользователя не должно быть пустым.".to_string())
        );
    }

    #[test]
    fn validate_registration_rejects_invalid_email() {
        assert_eq!(
            validate_registration("alice", "invalid", "password"),
            Some("Введите корректную почту.".to_string())
        );
    }

    #[test]
    fn validate_login_rejects_short_password() {
        assert_eq!(
            validate_login("alice", "short"),
            Some("Пароль должен содержать минимум 8 символов.".to_string())
        );
    }

    #[test]
    fn validate_post_rejects_empty_content() {
        assert_eq!(
            validate_post("Заголовок", "   "),
            Some("Текст поста не должен быть пустым.".to_string())
        );
    }

    #[test]
    fn validate_post_accepts_valid_input() {
        assert_eq!(validate_post("Заголовок", "Текст"), None);
    }
}
