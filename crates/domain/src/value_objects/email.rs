use regex::Regex;
use serde::{Deserialize, Serialize};
use shared::AppError;
use std::sync::OnceLock;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("Invalid email regex")
    })
}

/// Email value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Email(String);

impl Email {
    /// Create a new email with validation
    pub fn new(email: impl Into<String>) -> Result<Self, AppError> {
        let email = email.into();
        Self::validate(&email)?;
        Ok(Self(email.to_lowercase()))
    }

    /// Validate email format
    fn validate(email: &str) -> Result<(), AppError> {
        if email.is_empty() {
            return Err(AppError::InvalidEmail("Email cannot be empty".to_string()));
        }

        if email.len() > 255 {
            return Err(AppError::InvalidEmail(
                "Email cannot exceed 255 characters".to_string(),
            ));
        }

        if !get_email_regex().is_match(email) {
            return Err(AppError::InvalidEmail(format!(
                "Invalid email format: {}",
                email
            )));
        }

        Ok(())
    }

    /// Get the email as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the domain part of the email
    pub fn domain(&self) -> Option<&str> {
        self.0.split('@').nth(1)
    }

    /// Get the local part of the email
    pub fn local_part(&self) -> Option<&str> {
        self.0.split('@').next()
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(Email::new("user@example.com").is_ok());
        assert!(Email::new("user.name@example.com").is_ok());
        assert!(Email::new("user+tag@example.co.uk").is_ok());
    }

    #[test]
    fn test_invalid_email() {
        assert!(Email::new("").is_err());
        assert!(Email::new("invalid").is_err());
        assert!(Email::new("@example.com").is_err());
        assert!(Email::new("user@").is_err());
    }

    #[test]
    fn test_email_normalization() {
        let email = Email::new("USER@EXAMPLE.COM").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_parts() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.local_part(), Some("user"));
        assert_eq!(email.domain(), Some("example.com"));
    }
}
