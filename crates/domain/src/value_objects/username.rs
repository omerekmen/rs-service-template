use regex::Regex;
use serde::{Deserialize, Serialize};
use shared::AppError;
use std::sync::OnceLock;

static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_username_regex() -> &'static Regex {
    USERNAME_REGEX
        .get_or_init(|| Regex::new(r"^[a-zA-Z0-9_-]{3,30}$").expect("Invalid username regex"))
}

/// Username value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Username(String);

impl Username {
    /// Create a new username with validation
    pub fn new(username: impl Into<String>) -> Result<Self, AppError> {
        let username = username.into();
        Self::validate(&username)?;
        Ok(Self(username))
    }

    /// Validate username format
    fn validate(username: &str) -> Result<(), AppError> {
        if username.is_empty() {
            return Err(AppError::InvalidUsername(
                "Username cannot be empty".to_string(),
            ));
        }

        if username.len() < 3 {
            return Err(AppError::InvalidUsername(
                "Username must be at least 3 characters".to_string(),
            ));
        }

        if username.len() > 30 {
            return Err(AppError::InvalidUsername(
                "Username cannot exceed 30 characters".to_string(),
            ));
        }

        if !get_username_regex().is_match(username) {
            return Err(AppError::InvalidUsername(
                "Username can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        Ok(())
    }

    /// Get the username as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username() {
        assert!(Username::new("user123").is_ok());
        assert!(Username::new("user_name").is_ok());
        assert!(Username::new("user-name").is_ok());
        assert!(Username::new("User123").is_ok());
    }

    #[test]
    fn test_invalid_username() {
        assert!(Username::new("").is_err());
        assert!(Username::new("ab").is_err()); // Too short
        assert!(Username::new("a".repeat(31)).is_err()); // Too long
        assert!(Username::new("user name").is_err()); // Contains space
        assert!(Username::new("user@name").is_err()); // Contains special char
    }
}
