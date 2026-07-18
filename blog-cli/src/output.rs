//! Форматирование вывода CLI.

use anyhow::Error;
use blog_client::BlogClientError;
use blog_client::models::{AuthResponse, Post, PostPage};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorOutput {
    error: &'static str,
    message: String,
}

/// Печатает результат аутентификации.
///
/// # Errors
///
/// Возвращает ошибку, если JSON-ответ не удалось сериализовать.
pub(crate) fn print_auth(response: &AuthResponse, json: bool) -> anyhow::Result<()> {
    if json {
        print_json(response)
    } else {
        println!("Logged in as {}", response.user.username);
        Ok(())
    }
}

/// Печатает пост.
///
/// # Errors
///
/// Возвращает ошибку, если JSON-ответ не удалось сериализовать.
pub(crate) fn print_post(post: &Post, json: bool) -> anyhow::Result<()> {
    if json {
        print_json(post)
    } else {
        println!("#{} {}", post.id, post.title);
        println!("Author: {}", post.author_id);
        println!("Created: {}", post.created_at);
        if let Some(updated_at) = post.updated_at {
            println!("Updated: {updated_at}");
        }
        println!();
        println!("{}", post.content);
        Ok(())
    }
}

/// Печатает страницу постов.
///
/// # Errors
///
/// Возвращает ошибку, если JSON-ответ не удалось сериализовать.
pub(crate) fn print_post_page(page: &PostPage, json: bool) -> anyhow::Result<()> {
    if json {
        print_json(page)
    } else {
        println!(
            "Posts: {} of {} (limit {}, offset {})",
            page.posts.len(),
            page.total,
            page.limit,
            page.offset
        );

        for post in &page.posts {
            println!("#{} {}", post.id, post.title);
        }

        Ok(())
    }
}

/// Печатает результат удаления поста.
///
/// # Errors
///
/// Возвращает ошибку, если JSON-ответ не удалось сериализовать.
pub(crate) fn print_deleted(id: i64, json: bool) -> anyhow::Result<()> {
    if json {
        print_json(&serde_json::json!({ "deleted": true, "id": id }))
    } else {
        println!("Post {id} deleted");
        Ok(())
    }
}

/// Печатает ошибку CLI.
pub(crate) fn print_error(error: &Error, json: bool) {
    if !json {
        eprintln!("Error: {error}");
        return;
    }

    let output = error_output(error);
    let serialized = serde_json::to_string_pretty(&output).unwrap_or_else(|_| {
        r#"{"error":"error","message":"failed to serialize error"}"#.to_string()
    });

    eprintln!("{serialized}");
}

fn print_json<T>(value: &T) -> anyhow::Result<()>
where
    T: Serialize,
{
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn error_output(error: &Error) -> ErrorOutput {
    if let Some(client_error) = error.downcast_ref::<BlogClientError>() {
        return match client_error {
            BlogClientError::Http(_) => ErrorOutput {
                error: "http error",
                message: client_error.to_string(),
            },
            BlogClientError::GrpcStatus(_) | BlogClientError::GrpcTransport(_) => ErrorOutput {
                error: "grpc error",
                message: client_error.to_string(),
            },
            BlogClientError::NotFound => ErrorOutput {
                error: "not found",
                message: client_error.to_string(),
            },
            BlogClientError::Unauthorized => ErrorOutput {
                error: "unauthorized",
                message: client_error.to_string(),
            },
            BlogClientError::Forbidden => ErrorOutput {
                error: "forbidden",
                message: client_error.to_string(),
            },
            BlogClientError::Conflict(message) => ErrorOutput {
                error: "conflict",
                message: message.clone(),
            },
            BlogClientError::InvalidRequest(message) => ErrorOutput {
                error: "invalid request",
                message: message.clone(),
            },
            BlogClientError::MissingToken => ErrorOutput {
                error: "token is required",
                message: client_error.to_string(),
            },
        };
    }

    ErrorOutput {
        error: "error",
        message: error.to_string(),
    }
}
