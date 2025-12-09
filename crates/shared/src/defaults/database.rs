/// Default values for the database configuration.

pub const DEFAULT_DATABASE_SYSTEM: &str = "postgresql";
pub const DEFAULT_DATABASE_CONNECTION_STRING: &str = "postgresql://postgres:postgres@localhost/auth_service";
pub const DEFAULT_DATABASE_MAX_CONNECTIONS: u32 = 20;
pub const DEFAULT_DATABASE_MIN_CONNECTIONS: u32 = 5;
pub const DEFAULT_DATABASE_CONNECTION_TIMEOUT_SECONDS: u64 = 30;
pub const DEFAULT_DATABASE_IDLE_TIMEOUT_SECONDS: u64 = 600;
pub const DEFAULT_DATABASE_MAX_LIFETIME_SECONDS: u64 = 1800;
pub const DEFAULT_DATABASE_ENABLE_LOGGING: bool = false;
pub const DEFAULT_DATABASE_RUN_MIGRATIONS: bool = true;
