pub mod error;
pub mod tracing_setup;

pub type AppResult<T> = Result<T, error::AppError>;
