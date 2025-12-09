//! Default cache configuration values

pub const DEFAULT_CACHE_SYSTEM: &str = "redis";
pub const DEFAULT_CACHE_URL: &str = "redis://localhost:6379";
pub const DEFAULT_CACHE_POOL_SIZE: usize = 10;
pub const DEFAULT_CACHE_MIN_CONNECTIONS: u32 = 2;
pub const DEFAULT_CACHE_MAX_CONNECTIONS: u32 = 10;
pub const DEFAULT_CACHE_CONNECTION_TIMEOUT_SECONDS: u64 = 5;
pub const DEFAULT_CACHE_IDLE_TIMEOUT_SECONDS: u64 = 300;
pub const DEFAULT_CACHE_MAX_LIFETIME_SECONDS: u64 = 1800;
pub const DEFAULT_CACHE_ENABLE_LOGGING: bool = false;
