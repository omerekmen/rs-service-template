use serde::{Deserialize};
use super::{
    ServerConfig, DatabaseConfig, CacheConfig, JwtConfig, OAuthConfig, EmailConfig,
    SecurityConfig, LoggingConfig, FeatureFlags, EventPublisherConfig
};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub event_publisher: EventPublisherConfig,
    pub jwt: JwtConfig,
    pub oauth: OAuthConfig,
    pub email: EmailConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
    pub features: FeatureFlags,
}

impl AppConfig {
    pub fn load(env: &str) -> Result<Self, std::io::Error> {
        
        Ok(AppConfig {
            server: ServerConfig::load(&env)?,
            database: DatabaseConfig::load(&env)?,
            redis: RedisConfig::load(&env)?,
            event_publisher: EventPublisherConfig::load(&env)?,
            jwt: JwtConfig::load(&env)?,
            oauth: OAuthConfig::default(),
            email: EmailConfig::load(&env)?,
            security: SecurityConfig::load(&env)?,
            logging: LoggingConfig::load(&env)?,
            features: FeatureFlags::load(&env)?,
        })
    }
}
