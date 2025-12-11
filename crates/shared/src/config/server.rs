use serde::Deserialize;

use crate::defaults::server::*;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub request_timeout_seconds: u64,
    pub keep_alive_seconds: u64,
    pub max_connections: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            workers: DEFAULT_WORKERS,
            request_timeout_seconds: DEFAULT_REQUEST_TIMEOUT_SECONDS,
            keep_alive_seconds: DEFAULT_KEEP_ALIVE_SECONDS,
            max_connections: DEFAULT_MAX_CONNECTIONS,
        }
    }
}

impl ServerConfig {
    pub fn load(env: &str) -> Result<Self, config::ConfigError> {
        let default: ServerConfig = Self::default();
        let builder = config::Config::builder()
            .set_default("server.host", default.host.clone())?
            .set_default("server.port", default.port)?
            .set_default("server.workers", default.workers as i64)?
            .set_default(
                "server.request_timeout_seconds",
                default.request_timeout_seconds,
            )?
            .set_default("server.keep_alive_seconds", default.keep_alive_seconds)?
            .set_default("server.max_connections", default.max_connections as i64)?;

        let config = builder
            .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("__")
                    .separator("__")
            )
            .build()?;

        config.get::<ServerConfig>("server")
    }
}
