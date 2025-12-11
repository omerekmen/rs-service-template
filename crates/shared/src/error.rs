use std::fmt;

/// Application result type alias
pub type AppResult<T> = Result<T, AppError>;

/// Application-wide error type
#[derive(Debug)]
pub enum AppError {
    // Domain errors
    ValidationError(String),
    InvalidEmail(String),
    InvalidUsername(String),

    // Application errors
    NotFound(String),
    AlreadyExists(String),
    Unauthorized(String),
    Forbidden(String),

    // Infrastructure errors
    DatabaseError(String),
    CacheError(String),

    // Internal errors
    InternalError(String),
    ConfigurationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InvalidEmail(msg) => write!(f, "Invalid email: {}", msg),
            AppError::InvalidUsername(msg) => write!(f, "Invalid username: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::CacheError(msg) => write!(f, "Cache error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Conversions from infrastructure errors
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                // Check for unique constraint violations
                if let Some(code) = db_err.code() {
                    if code == "23505" {
                        // PostgreSQL unique violation
                        return AppError::AlreadyExists("Resource already exists".to_string());
                    }
                }
                AppError::DatabaseError(db_err.to_string())
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        AppError::CacheError(err.to_string())
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::CacheError(err.to_string())
    }
}

#[cfg(feature = "actix-integration")]
impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidEmail(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidUsername(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::CacheError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConfigurationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        let status = self.status_code();
        let error_message = self.to_string();

        HttpResponse::build(status).json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16(),
            }
        }))
    }
}
