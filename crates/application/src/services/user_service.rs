use shared::{AppError, AppResult, UserId};
use std::sync::Arc;

use domain::{Email, User, UserRepository, Username};

use crate::dtos::{CreateUserRequest, UpdateUserRequest, UserListResponse, UserResponse};

/// User service containing all user-related use cases
///
/// This service orchestrates domain logic and repository operations.
/// It follows the Application Service pattern from Clean Architecture.
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    /// Create a new UserService
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Use Case: Create a new user
    ///
    /// Business rules:
    /// - Username must be unique
    /// - Email must be unique
    /// - Username and email must be valid
    pub async fn create_user(&self, request: CreateUserRequest) -> AppResult<UserResponse> {
        // Validate and create value objects
        let username = Username::new(request.username)?;
        let email = Email::new(request.email)?;

        // Business rule: Username must be unique
        if self.user_repository.username_exists(&username).await? {
            return Err(AppError::AlreadyExists(format!(
                "Username '{}' already exists",
                username
            )));
        }

        // Business rule: Email must be unique
        if self.user_repository.email_exists(&email).await? {
            return Err(AppError::AlreadyExists(format!(
                "Email '{}' already exists",
                email
            )));
        }

        // Create domain entity
        let mut user = User::new(username, email);

        // Set optional fields
        if let Some(full_name) = request.full_name {
            user.update_full_name(Some(full_name))?;
        }

        // Persist user
        self.user_repository.create(&user).await?;

        Ok(UserResponse::from(user))
    }

    /// Use Case: Get user by ID
    pub async fn get_user(&self, user_id: UserId) -> AppResult<UserResponse> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;

        Ok(UserResponse::from(user))
    }

    /// Use Case: Get user by username
    pub async fn get_user_by_username(&self, username: String) -> AppResult<UserResponse> {
        let username = Username::new(username)?;
        let user = self
            .user_repository
            .find_by_username(&username)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("User with username '{}' not found", username))
            })?;

        Ok(UserResponse::from(user))
    }

    /// Use Case: Update user
    pub async fn update_user(
        &self,
        user_id: UserId,
        request: UpdateUserRequest,
    ) -> AppResult<UserResponse> {
        // Retrieve existing user
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;

        // Update username if provided
        if let Some(username_str) = request.username {
            let new_username = Username::new(username_str)?;

            // Check if username is already taken by another user
            if let Some(existing_user) =
                self.user_repository.find_by_username(&new_username).await?
            {
                if existing_user.id() != user_id {
                    return Err(AppError::AlreadyExists(format!(
                        "Username '{}' already exists",
                        new_username
                    )));
                }
            }

            user.update_username(new_username);
        }

        // Update email if provided
        if let Some(email_str) = request.email {
            let new_email = Email::new(email_str)?;

            // Check if email is already taken by another user
            if let Some(existing_user) = self.user_repository.find_by_email(&new_email).await? {
                if existing_user.id() != user_id {
                    return Err(AppError::AlreadyExists(format!(
                        "Email '{}' already exists",
                        new_email
                    )));
                }
            }

            user.update_email(new_email);
        }

        // Update full name if provided (even if None to allow clearing)
        if request.full_name.is_some() {
            user.update_full_name(request.full_name)?;
        }

        // Persist changes
        self.user_repository.update(&user).await?;

        Ok(UserResponse::from(user))
    }

    /// Use Case: Delete user
    pub async fn delete_user(&self, user_id: UserId) -> AppResult<()> {
        // Verify user exists
        let _user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;

        // Delete user
        self.user_repository.delete(user_id).await?;

        Ok(())
    }

    /// Use Case: List users with pagination
    pub async fn list_users(&self, limit: i64, offset: i64) -> AppResult<UserListResponse> {
        // Validate pagination parameters
        if !(1..=100).contains(&limit) {
            return Err(AppError::ValidationError(
                "Limit must be between 1 and 100".to_string(),
            ));
        }

        if offset < 0 {
            return Err(AppError::ValidationError(
                "Offset must be non-negative".to_string(),
            ));
        }

        // Fetch users and total count
        let users = self.user_repository.list(limit, offset).await?;
        let total = self.user_repository.count().await?;

        Ok(UserListResponse {
            users: users.into_iter().map(UserResponse::from).collect(),
            total,
            limit,
            offset,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock repository for testing
    struct MockUserRepository {
        users: Mutex<HashMap<UserId, User>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: &User) -> AppResult<()> {
            self.users.lock().unwrap().insert(user.id(), user.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>> {
            Ok(self.users.lock().unwrap().get(&id).cloned())
        }

        async fn find_by_username(&self, username: &Username) -> AppResult<Option<User>> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .values()
                .find(|u| u.username() == username)
                .cloned())
        }

        async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .values()
                .find(|u| u.email() == email)
                .cloned())
        }

        async fn update(&self, user: &User) -> AppResult<()> {
            self.users.lock().unwrap().insert(user.id(), user.clone());
            Ok(())
        }

        async fn delete(&self, id: UserId) -> AppResult<()> {
            self.users.lock().unwrap().remove(&id);
            Ok(())
        }

        async fn username_exists(&self, username: &Username) -> AppResult<bool> {
            Ok(self.find_by_username(username).await?.is_some())
        }

        async fn email_exists(&self, email: &Email) -> AppResult<bool> {
            Ok(self.find_by_email(email).await?.is_some())
        }

        async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<User>> {
            let users = self.users.lock().unwrap();
            Ok(users
                .values()
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }

        async fn count(&self) -> AppResult<i64> {
            Ok(self.users.lock().unwrap().len() as i64)
        }
    }

    #[tokio::test]
    async fn test_create_user() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserService::new(repo);

        let request = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            full_name: Some("Test User".to_string()),
        };

        let result = service.create_user(request).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_create_duplicate_username() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserService::new(repo);

        let request1 = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test1@example.com".to_string(),
            full_name: None,
        };

        service.create_user(request1).await.unwrap();

        let request2 = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test2@example.com".to_string(),
            full_name: None,
        };

        let result = service.create_user(request2).await;
        assert!(matches!(result, Err(AppError::AlreadyExists(_))));
    }
}
