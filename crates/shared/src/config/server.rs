use serde::{Deserialize};

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
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: num_cpus::get(),
            request_timeout_seconds: 60,
            keep_alive_seconds: 75,
            max_connections: 25000,
        }
    }
}

impl ServerConfig {
    /// Load configuration from environment variables and config files
    pub fn load(env: &str) -> Result<Self, std::io::Error> {
        let builder = config::Config::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("server.workers", num_cpus::get() as i64)?
            .set_default("server.request_timeout_seconds", 60)?
            .set_default("server.keep_alive_seconds", 75)?
            .set_default("server.max_connections", 25000)?;

        let config = builder
            .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
            .add_source(config::Environment::with_prefix("APP__SERVER").separator("__"))
            .build()?;

        config
            .get::<ServerConfig>("server")
            .or_else(|_| config.try_deserialize())
    }
}
