use deadpool_redis::{Config, CreatePoolError, Pool, Runtime};
use shared::config::cache::CacheConfig;

pub async fn create_redis_pool(config: CacheConfig) -> Result<Pool, CreatePoolError> {
    let cfg = Config::from_url(config.url.clone());

    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    Ok(pool)
}
