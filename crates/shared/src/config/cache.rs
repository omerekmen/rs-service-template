use serde::Deserialize;

use crate::defaults::cache;

/// Cache (Redis) configuration
#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    pub url: String,
    pub pool_size: usize,
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            url: cache::DEFAULT_CACHE_URL.to_string(),
            pool_size: cache::DEFAULT_CACHE_POOL_SIZE,
            min_connections: cache::DEFAULT_CACHE_MIN_CONNECTIONS,
            max_connections: cache::DEFAULT_CACHE_MAX_CONNECTIONS,
            connection_timeout_seconds: cache::DEFAULT_CACHE_CONNECTION_TIMEOUT_SECONDS,
            idle_timeout_seconds: cache::DEFAULT_CACHE_IDLE_TIMEOUT_SECONDS,
            max_lifetime_seconds: cache::DEFAULT_CACHE_MAX_LIFETIME_SECONDS,
        }
    }
}   

impl CacheConfig {
    /// Load configuration from environment variables and config files
    pub fn load(env: &str) -> Result<Self, config::ConfigError> {
        let default: CacheConfig = Self::default();
        let builder = config::Config::builder()
            .set_default("cache.url", default.url)?
            .set_default("cache.pool_size", default.pool_size as i64)?
            .set_default("cache.min_connections", default.min_connections)?
            .set_default("cache.max_connections", default.max_connections)?
            .set_default("cache.connection_timeout_seconds", default.connection_timeout_seconds)?
            .set_default("cache.idle_timeout_seconds", default.idle_timeout_seconds)?
            .set_default("cache.max_lifetime_seconds", default.max_lifetime_seconds)?;

        let config = builder
            .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
            .add_source(config::Environment::with_prefix("APP__CACHE").separator("__"))
            .build()?;

        config
            .get::<CacheConfig>("cache")
            .or_else(|_| config.try_deserialize())
    }
}
