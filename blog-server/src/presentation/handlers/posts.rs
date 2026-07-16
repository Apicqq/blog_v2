//! HTTP-handlers постов блога.

use actix_web::{HttpResponse, delete, get, guard, post, put, web};
use validator::Validate;

use crate::application::blog_service::BlogService;
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::repositories::sea_orm_post_repository::SeaOrmPostRepository;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::post::{
    CreatePostRequest, ListPostsQuery, ListPostsResponse, PostResponse, UpdatePostRequest,
};

type BlogPostService = BlogService<SeaOrmPostRepository>;

/// Настраивает публичные маршруты постов.
pub fn configure_public_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .guard(guard::Get())
            .service(list_posts_handler)
            .service(get_post_handler),
    );
}

/// Настраивает защищенные маршруты постов.
pub fn configure_protected_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(create_post_handler)
            .service(update_post_handler)
            .service(delete_post_handler),
    );
}

/// Возвращает страницу постов.
#[get("")]
async fn list_posts_handler(
    service: web::Data<BlogPostService>,
    query: web::Query<ListPostsQuery>,
) -> Result<HttpResponse, DomainError> {
    let query = query.into_inner();
    let limit = query.limit();
    let offset = query.offset();
    let page = service.list_posts(limit, offset).await?;

    Ok(HttpResponse::Ok().json(ListPostsResponse::new(page, limit, offset)))
}

/// Возвращает пост по идентификатору.
#[get("/{post_id}")]
async fn get_post_handler(
    service: web::Data<BlogPostService>,
    post_id: web::Path<i64>,
) -> Result<HttpResponse, DomainError> {
    let post = service.get_post(post_id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

/// Создает новый пост от имени текущего пользователя.
#[post("")]
async fn create_post_handler(
    user: AuthenticatedUser,
    service: web::Data<BlogPostService>,
    payload: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, DomainError> {
    let payload = payload.into_inner();
    payload
        .validate()
        .map_err(|err| DomainError::Validation(err.to_string()))?;
    let post = service
        .create_post(
            user.user_id,
            payload.title.trim().to_string(),
            payload.content,
        )
        .await?;

    Ok(HttpResponse::Created().json(PostResponse::from(post)))
}

/// Обновляет пост текущего пользователя.
#[put("/{post_id}")]
async fn update_post_handler(
    user: AuthenticatedUser,
    service: web::Data<BlogPostService>,
    post_id: web::Path<i64>,
    payload: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, DomainError> {
    let payload = payload.into_inner();
    payload
        .validate()
        .map_err(|err| DomainError::Validation(err.to_string()))?;
    let post = service
        .update_post(user.user_id, post_id.into_inner(), payload.into())
        .await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

/// Удаляет пост текущего пользователя.
#[delete("/{post_id}")]
async fn delete_post_handler(
    user: AuthenticatedUser,
    service: web::Data<BlogPostService>,
    post_id: web::Path<i64>,
) -> Result<HttpResponse, DomainError> {
    service
        .delete_post(user.user_id, post_id.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
