//! gRPC-представление серверного приложения.

use uuid::Uuid;

use blog_proto::generated::blog_service_server::BlogService as BlogServiceTrait;
use blog_proto::generated::{
    AuthResponse, CreatePostRequest, DeletePostRequest, DeletePostResponse, GetPostRequest,
    ListPostsRequest, ListPostsResponse, LoginRequest, Post, RegisterRequest, UpdatePostRequest,
    User,
};
use tonic::{Request, Response, Status};
use tracing::{debug, warn};

use crate::application::auth_service::{AuthService, AuthSession};
use crate::application::blog_service::{BlogService, PostPage};
use crate::application::ports::token_service::TokenService;
use crate::domain::errors::DomainError;
use crate::domain::post::UpdatePost;
use crate::domain::user::{LoginCredentials, RegistrationData};
use crate::infrastructure::persistence::repositories::sea_orm_post_repository::SeaOrmPostRepository;
use crate::infrastructure::persistence::repositories::sea_orm_user_repository::SeaOrmUserRepository;
use crate::infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;
use crate::infrastructure::security::jwt_token_service::JwtTokenService;

const DEFAULT_POSTS_LIMIT: u64 = 10;
const MAX_POSTS_LIMIT: u64 = 100;

/// Конкретный сервис аутентификации серверного приложения.
type BlogAuthService = AuthService<SeaOrmUserRepository, Argon2PasswordHasher, JwtTokenService>;
/// Конкретный сервис постов серверного приложения.
type BlogPostService = BlogService<SeaOrmPostRepository>;

/// gRPC-сервис блога.
#[derive(Debug, Clone)]
pub struct BlogGrpcApi {
    auth: BlogAuthService,
    blog: BlogPostService,
    tokens: JwtTokenService,
}

impl BlogGrpcApi {
    /// Создает gRPC-сервис блога.
    #[must_use]
    pub const fn new(
        auth: BlogAuthService,
        blog: BlogPostService,
        tokens: JwtTokenService,
    ) -> Self {
        Self { auth, blog, tokens }
    }
}

#[tonic::async_trait]
impl BlogServiceTrait for BlogGrpcApi {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let request = request.into_inner();
        let registration =
            RegistrationData::new(&request.username, &request.email, request.password)
                .map_err(status_from_domain_error)?;
        let session = self
            .auth
            .register(registration)
            .await
            .map_err(status_from_domain_error)?;

        Ok(Response::new(auth_response_from_session(session)))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let request = request.into_inner();
        let credentials = LoginCredentials::new(&request.username, request.password)
            .map_err(status_from_domain_error)?;
        let session = self
            .auth
            .login(credentials)
            .await
            .map_err(status_from_domain_error)?;

        Ok(Response::new(auth_response_from_session(session)))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let user_id = authenticated_user_id(&request, &self.tokens)?;
        let request = request.into_inner();
        let post = self
            .blog
            .create_post(user_id, request.title, request.content)
            .await
            .map_err(status_from_domain_error)?;

        Ok(Response::new(post.into()))
    }

    async fn get_post(&self, request: Request<GetPostRequest>) -> Result<Response<Post>, Status> {
        let request = request.into_inner();
        let post = self
            .blog
            .get_post(request.id)
            .await
            .map_err(status_from_domain_error)?;

        Ok(Response::new(post.into()))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let user_id = authenticated_user_id(&request, &self.tokens)?;
        let request = request.into_inner();
        let update =
            UpdatePost::new(&request.title, request.content).map_err(status_from_domain_error)?;
        let post = self
            .blog
            .update_post(user_id, request.id, update)
            .await
            .map_err(status_from_domain_error)?;

        Ok(Response::new(post.into()))
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let user_id = authenticated_user_id(&request, &self.tokens)?;
        let request = request.into_inner();
        self.blog
            .delete_post(user_id, request.id)
            .await
            .map_err(status_from_domain_error)?;

        Ok(Response::new(DeletePostResponse {}))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let request = request.into_inner();
        let limit = normalized_limit(request.limit);
        let offset = request.offset;
        let page = self
            .blog
            .list_posts(limit, offset)
            .await
            .map_err(status_from_domain_error)?;

        Ok(Response::new(list_posts_response_from_page(
            page, limit, offset,
        )))
    }
}

fn authenticated_user_id<T>(
    request: &Request<T>,
    token_service: &JwtTokenService,
) -> Result<Uuid, Status> {
    let header = request.metadata().get("authorization").ok_or_else(|| {
        warn!("gRPC authorization metadata is missing");
        Status::unauthenticated("missing authorization metadata")
    })?;
    let header = header.to_str().map_err(|_| {
        warn!("gRPC authorization metadata is not valid ASCII");
        Status::unauthenticated("invalid authorization metadata")
    })?;
    let token = header
        .strip_prefix("Bearer ")
        .filter(|token| !token.is_empty())
        .ok_or_else(|| {
            warn!("gRPC authorization metadata has invalid bearer format");
            Status::unauthenticated("invalid authorization metadata")
        })?;

    let user_id = token_service.verify(token).map_err(|_| {
        warn!("gRPC authorization token rejected");
        Status::unauthenticated("invalid authorization token")
    })?;

    debug!(user_id = %user_id, "gRPC authorization token accepted");

    Ok(user_id)
}

fn normalized_limit(limit: u64) -> u64 {
    if limit == 0 {
        DEFAULT_POSTS_LIMIT
    } else {
        limit.min(MAX_POSTS_LIMIT)
    }
}

fn auth_response_from_session(session: AuthSession) -> AuthResponse {
    AuthResponse {
        token: session.token,
        user: Some(session.user.into()),
    }
}

fn list_posts_response_from_page(page: PostPage, limit: u64, offset: u64) -> ListPostsResponse {
    ListPostsResponse {
        posts: page.posts.into_iter().map(Post::from).collect(),
        total: page.total,
        limit,
        offset,
    }
}

fn status_from_domain_error(error: DomainError) -> Status {
    match error {
        DomainError::Validation(message) => Status::invalid_argument(message),
        DomainError::Unauthorized | DomainError::InvalidCredentials => {
            Status::unauthenticated(error.to_string())
        }
        DomainError::Forbidden => Status::permission_denied(error.to_string()),
        DomainError::UserNotFound(_) | DomainError::PostNotFound(_) => {
            Status::not_found(error.to_string())
        }
        DomainError::UsernameAlreadyTaken | DomainError::EmailAlreadyTaken => {
            Status::already_exists(error.to_string())
        }
        DomainError::Internal(_) => Status::internal(error.to_string()),
    }
}

impl From<crate::domain::user::User> for User {
    fn from(user: crate::domain::user::User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
        }
    }
}

impl From<crate::domain::post::Post> for Post {
    fn from(post: crate::domain::post::Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id.to_string(),
            created_at: post.created_at.timestamp(),
            updated_at: post.updated_at.map(|updated_at| updated_at.timestamp()),
        }
    }
}
