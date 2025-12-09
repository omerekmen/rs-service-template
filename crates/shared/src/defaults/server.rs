//! Server default configurations

pub const DEFAULT_HOST: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 8080;
pub const DEFAULT_WORKERS: usize = 4;
pub const DEFAULT_REQUEST_TIMEOUT_SECONDS: u64 = 60;
pub const DEFAULT_KEEP_ALIVE_SECONDS: u64 = 75;
pub const DEFAULT_MAX_CONNECTIONS: usize = 25000;
