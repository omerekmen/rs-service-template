use actix_web::{HttpResponse, Result, web};
use serde::Deserialize;

use application::{CreateUserRequest, UpdateUserRequest, UserService};
use shared::{AppError, UserId};

/// Query parameters for user listing
#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

/// POST /api/v1/users - Create a new user
pub async fn create_user(
    service: web::Data<UserService>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    let user = service.create_user(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(user))
}

/// GET /api/v1/users/:id - Get user by ID
pub async fn get_user(
    service: web::Data<UserService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let user_id_str = path.into_inner();
    let user_id = uuid::Uuid::parse_str(&user_id_str)
        .map_err(|_| AppError::ValidationError("Invalid user ID format".to_string()))?;

    let user = service.get_user(UserId::from_uuid(user_id)).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// GET /api/v1/users/username/:username - Get user by username
pub async fn get_user_by_username(
    service: web::Data<UserService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let username = path.into_inner();
    let user = service.get_user_by_username(username).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// PUT /api/v1/users/:id - Update user
pub async fn update_user(
    service: web::Data<UserService>,
    path: web::Path<String>,
    request: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    let user_id_str = path.into_inner();
    let user_id = uuid::Uuid::parse_str(&user_id_str)
        .map_err(|_| AppError::ValidationError("Invalid user ID format".to_string()))?;

    let user = service
        .update_user(UserId::from_uuid(user_id), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(user))
}

/// DELETE /api/v1/users/:id - Delete user
pub async fn delete_user(
    service: web::Data<UserService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let user_id_str = path.into_inner();
    let user_id = uuid::Uuid::parse_str(&user_id_str)
        .map_err(|_| AppError::ValidationError("Invalid user ID format".to_string()))?;

    service.delete_user(UserId::from_uuid(user_id)).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// GET /api/v1/users - List users with pagination
pub async fn list_users(
    service: web::Data<UserService>,
    query: web::Query<ListUsersQuery>,
) -> Result<HttpResponse> {
    let users = service.list_users(query.limit, query.offset).await?;
    Ok(HttpResponse::Ok().json(users))
}
