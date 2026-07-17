use actix_web::middleware::DefaultHeaders;

/// Собирает базовые защитные HTTP-заголовки.
#[must_use]
pub fn default_security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("Referrer-Policy", "no-referrer"))
        .add(("Permissions-Policy", "geolocation=()"))
        .add(("Cross-Origin-Opener-Policy", "same-origin"))
}
