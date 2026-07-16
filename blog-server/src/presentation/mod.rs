//! HTTP-представление серверного приложения.

pub mod auth;
/// DTO HTTP-слоя.
pub mod dto;
/// Преобразование доменных ошибок в HTTP-ответы.
pub mod errors;
/// HTTP-handlers серверного приложения.
pub mod handlers;
pub mod middlewares;
