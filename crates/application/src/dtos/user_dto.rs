use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::UserId;

use domain::{User, UserStatus};

/// Request DTO for creating a user
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
}

/// Request DTO for updating a user
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
}

/// Response DTO for user data
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            username: user.username().to_string(),
            email: user.email().to_string(),
            full_name: user.full_name().map(|s| s.to_string()),
            status: user.status(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}

/// List response with pagination info
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
