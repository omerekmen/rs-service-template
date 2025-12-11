use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{AppError, UserId};

use crate::value_objects::{Email, Username};

/// User status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// User entity representing a user in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    username: Username,
    email: Email,
    full_name: Option<String>,
    status: UserStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// Create a new user
    pub fn new(username: Username, email: Email) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            username,
            email,
            full_name: None,
            status: UserStatus::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstruct user from database (used by infrastructure layer)
    pub fn from_persistence(
        id: UserId,
        username: Username,
        email: Email,
        full_name: Option<String>,
        status: UserStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            username,
            email,
            full_name,
            status,
            created_at,
            updated_at,
        }
    }

    /// Get user ID
    pub fn id(&self) -> UserId {
        self.id
    }

    /// Get username
    pub fn username(&self) -> &Username {
        &self.username
    }

    /// Get email
    pub fn email(&self) -> &Email {
        &self.email
    }

    /// Get full name
    pub fn full_name(&self) -> Option<&str> {
        self.full_name.as_deref()
    }

    /// Get status
    pub fn status(&self) -> UserStatus {
        self.status
    }

    /// Get created at timestamp
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Get updated at timestamp
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Update username
    pub fn update_username(&mut self, username: Username) {
        self.username = username;
        self.updated_at = Utc::now();
    }

    /// Update email
    pub fn update_email(&mut self, email: Email) {
        self.email = email;
        self.updated_at = Utc::now();
    }

    /// Update full name
    pub fn update_full_name(&mut self, full_name: Option<String>) -> Result<(), AppError> {
        if let Some(ref name) = full_name {
            if name.len() > 100 {
                return Err(AppError::ValidationError(
                    "Full name cannot exceed 100 characters".to_string(),
                ));
            }
        }
        self.full_name = full_name;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Activate user
    pub fn activate(&mut self) {
        self.status = UserStatus::Active;
        self.updated_at = Utc::now();
    }

    /// Deactivate user
    pub fn deactivate(&mut self) {
        self.status = UserStatus::Inactive;
        self.updated_at = Utc::now();
    }

    /// Suspend user
    pub fn suspend(&mut self) {
        self.status = UserStatus::Suspended;
        self.updated_at = Utc::now();
    }

    /// Check if user is active
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(username.clone(), email.clone());

        assert_eq!(user.username(), &username);
        assert_eq!(user.email(), &email);
        assert_eq!(user.status(), UserStatus::Active);
        assert!(user.is_active());
    }

    #[test]
    fn test_update_user() {
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        let mut user = User::new(username, email);

        let new_username = Username::new("newuser").unwrap();
        user.update_username(new_username.clone());
        assert_eq!(user.username(), &new_username);

        user.update_full_name(Some("Test User".to_string()))
            .unwrap();
        assert_eq!(user.full_name(), Some("Test User"));
    }

    #[test]
    fn test_user_status_changes() {
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        let mut user = User::new(username, email);

        assert!(user.is_active());

        user.suspend();
        assert_eq!(user.status(), UserStatus::Suspended);
        assert!(!user.is_active());

        user.activate();
        assert!(user.is_active());
    }
}
