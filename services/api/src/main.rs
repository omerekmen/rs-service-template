mod http_server;
pub mod route_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

    // Initialize tracing subscriber for logging
    // This reads the RUST_LOG environment variable to configure log levels
    // Example: RUST_LOG=debug,actix_web=info
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .with_target(true)  // Show the module path (e.g., api::http_server)
        .with_level(true)   // Show log level (INFO, ERROR, etc.)
        .with_thread_ids(env == "dev")    // Show thread IDs in dev mode
        .with_thread_names(env == "dev")  // Show thread names in dev mode
        .with_file(env == "dev")   // Don't show file name (can enable for debugging)
        .with_line_number(env == "dev")  // Don't show line numbers (can enable for debugging)
        .init();


    let config: shared::AppConfig = match shared::AppConfig::load(&env) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    tracing::info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    tracing::info!("Environment: {}", env);
    tracing::info!(
        "HTTP server will listen on {}:{}",
        config.server.host,
        config.server.port
    );

    let http_server: http_server::Server =
        http_server::Server::new(&config).await.map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create HTTP server: {}", e),
            )
        })?;

    tracing::info!("Server initialization complete. Starting HTTP server...");

    let http_result: std::io::Result<()> = http_server.run().await.map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("HTTP server error: {}", e),
        )
    });

    tracing::info!("Server shutdown complete");
    http_result
}
