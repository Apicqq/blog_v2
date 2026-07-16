//! HTTP-handlers постов блога.

use actix_web::{HttpResponse, web};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::application::blog_service::BlogService;
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::repositories::sea_orm_post_repository::SeaOrmPostRepository;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::post::{
    CreatePostRequest, ListPostsQuery, ListPostsResponse, PostResponse, UpdatePostRequest,
};
use crate::presentation::middlewares::jwt_auth::jwt_validator;

type BlogPostService = BlogService<SeaOrmPostRepository>;

/// Настраивает маршруты постов.
pub fn configure_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(
                web::resource("")
                    .route(web::get().to(list_posts_handler))
                    .route(
                        web::post()
                            .to(create_post_handler)
                            .wrap(HttpAuthentication::bearer(jwt_validator)),
                    ),
            )
            .service(
                web::resource("/{post_id}")
                    .route(web::get().to(get_post_handler))
                    .route(
                        web::put()
                            .to(update_post_handler)
                            .wrap(HttpAuthentication::bearer(jwt_validator)),
                    )
                    .route(
                        web::delete()
                            .to(delete_post_handler)
                            .wrap(HttpAuthentication::bearer(jwt_validator)),
                    ),
            ),
    );
}

/// Возвращает страницу постов.
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
async fn get_post_handler(
    service: web::Data<BlogPostService>,
    post_id: web::Path<i64>,
) -> Result<HttpResponse, DomainError> {
    let post = service.get_post(post_id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

/// Создает новый пост от имени текущего пользователя.
async fn create_post_handler(
    user: AuthenticatedUser,
    service: web::Data<BlogPostService>,
    payload: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, DomainError> {
    let payload = payload.into_inner();
    let post = service
        .create_post(user.user_id, payload.title, payload.content)
        .await?;

    Ok(HttpResponse::Created().json(PostResponse::from(post)))
}

/// Обновляет пост текущего пользователя.
async fn update_post_handler(
    user: AuthenticatedUser,
    service: web::Data<BlogPostService>,
    post_id: web::Path<i64>,
    payload: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, DomainError> {
    let payload = payload.into_inner();
    let post = service
        .update_post(user.user_id, post_id.into_inner(), payload.try_into()?)
        .await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

/// Удаляет пост текущего пользователя.
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
