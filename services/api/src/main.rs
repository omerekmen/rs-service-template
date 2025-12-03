mod http_server;
pub mod states;

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

    let http_server: http_server::Server = http_server::Server::new(&config)
        .map_err(|e: std::io::Error| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create HTTP server: {}", e)))?;

    let http_result: std::io::Result<()> = Ok(());

    http_result
}

