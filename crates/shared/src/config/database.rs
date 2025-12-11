use serde::Deserialize;

use crate::defaults::database;

/// Database (PostgreSQL) configuration
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub database_system: String,
    pub connection_string: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
    pub enable_logging: bool,
    pub run_migrations: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_system: database::DEFAULT_DATABASE_SYSTEM.to_string(),
            connection_string: database::DEFAULT_DATABASE_CONNECTION_STRING.to_string(),
            min_connections: database::DEFAULT_DATABASE_MIN_CONNECTIONS,
            max_connections: database::DEFAULT_DATABASE_MAX_CONNECTIONS,
            connection_timeout_seconds: database::DEFAULT_DATABASE_CONNECTION_TIMEOUT_SECONDS,
            idle_timeout_seconds: database::DEFAULT_DATABASE_IDLE_TIMEOUT_SECONDS,
            max_lifetime_seconds: database::DEFAULT_DATABASE_MAX_LIFETIME_SECONDS,
            enable_logging: database::DEFAULT_DATABASE_ENABLE_LOGGING,
            run_migrations: database::DEFAULT_DATABASE_RUN_MIGRATIONS,
        }
    }
}

impl DatabaseConfig {
    pub fn load(env: &str) -> Result<Self, config::ConfigError> {
        let default: DatabaseConfig = Self::default();
        let builder = config::Config::builder()
            .set_default("database.database_system", default.database_system.clone())?
            .set_default(
                "database.connection_string",
                default.connection_string.clone(),
            )?
            .set_default("database.min_connections", default.min_connections)?
            .set_default("database.max_connections", default.max_connections)?
            .set_default(
                "database.connection_timeout_seconds",
                default.connection_timeout_seconds,
            )?
            .set_default(
                "database.idle_timeout_seconds",
                default.idle_timeout_seconds,
            )?
            .set_default(
                "database.max_lifetime_seconds",
                default.max_lifetime_seconds,
            )?
            .set_default("database.enable_logging", default.enable_logging)?
            .set_default("database.run_migrations", default.run_migrations)?;

        let config = builder
            .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("__")
                    .separator("__")
            )
            .build()?;

        config.get::<DatabaseConfig>("database")
    }
}
