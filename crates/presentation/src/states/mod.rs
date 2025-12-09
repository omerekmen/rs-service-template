mod cache;
mod database;
mod email;
mod jwt;

use crate::states::{cache::CacheState, database::DatabaseState, email::EmailState, jwt::JwtState};

use infrastructure::{cache::redis::create_redis_pool, database::postgres::create_postgres_pool};

/// Application state shared across all handlers
#[derive(Clone, Default)]
pub struct AppState {
    pub db: DatabaseState,
    pub cache: CacheState,
    pub jwt: JwtState,
    pub email: EmailState,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn load(
        &mut self,
        conf: &shared::AppConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Load database configurations
        let db_pool = create_postgres_pool(conf.database.clone()).await?;
        self.db.add_db_pool("default".to_string(), db_pool);

        // Load cache configurations
        let cache = create_redis_pool(conf.cache.clone()).await?;
        self.cache.add_cache("default".to_string(), cache);

        Ok(self.clone())
    }
}
