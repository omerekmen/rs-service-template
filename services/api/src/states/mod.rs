use shared::{prelude::*, DatabaseConfig, RedisConfig, JwtConfig, EmailConfig};
use std::collections::HashMap;

/// Application state shared across all handlers
#[derive(Clone, Default)]
pub struct AppState {
    pub db_pools: HashMap<String, DatabaseConfig>,
    pub redis_configs: HashMap<String, RedisConfig>,
    pub jwt_configs: HashMap<String, JwtConfig>,
    pub email_configs: HashMap<String, EmailConfig>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, conf: &shared::AppConfig) {
        
    }

    pub fn add_db_pool(&mut self, name: String, config: DatabaseConfig) {
        self.db_pools.insert(name, config);
    }

    pub fn add_redis_config(&mut self, name: String, config: RedisConfig) {
        self.redis_configs.insert(name, config);
    }

    pub fn add_jwt_config(&mut self, name: String, config: JwtConfig) {
        self.jwt_configs.insert(name, config);
    }

    pub fn add_email_config(&mut self, name: String, config: EmailConfig) {
        self.email_configs.insert(name, config);
    }
}
