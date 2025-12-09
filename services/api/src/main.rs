mod http_server;
pub mod route_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

    let config: shared::AppConfig = match shared::AppConfig::load(&env) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    tracing::info!("Starting {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    tracing::info!("Environment: {}", env);
    tracing::info!("HTTP server will listen on {}:{}", config.server.host, config.server.port);

    let http_server: http_server::Server = http_server::Server::new(&config)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create HTTP server: {}", e)))?;

    tracing::info!("Server initialization complete. Starting HTTP server...");

    let http_result: std::io::Result<()> = http_server.run().await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("HTTP server error: {}", e)));
        
    tracing::info!("Server shutdown complete");
    http_result
}
