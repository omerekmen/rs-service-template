use async_trait::async_trait;
use shared::{AppResult, UserId};

use crate::entities::User;
use crate::value_objects::{Email, Username};

/// UserRepository trait (Port)
///
/// This trait defines the interface for user persistence operations.
/// It follows the Repository pattern and Dependency Inversion Principle.
/// The domain layer defines this interface (port), and the infrastructure
/// layer provides the concrete implementation (adapter).
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user
    async fn create(&self, user: &User) -> AppResult<()>;

    /// Find user by ID
    async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>>;

    /// Find user by username
    async fn find_by_username(&self, username: &Username) -> AppResult<Option<User>>;

    /// Find user by email
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;

    /// Update user
    async fn update(&self, user: &User) -> AppResult<()>;

    /// Delete user by ID
    async fn delete(&self, id: UserId) -> AppResult<()>;

    /// Check if username exists
    async fn username_exists(&self, username: &Username) -> AppResult<bool>;

    /// Check if email exists
    async fn email_exists(&self, email: &Email) -> AppResult<bool>;

    /// List all users with pagination
    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<User>>;

    /// Count total users
    async fn count(&self) -> AppResult<i64>;
}
