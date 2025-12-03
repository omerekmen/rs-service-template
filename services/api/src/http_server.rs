use actix_cors::Cors;
use actix_web::{
    web,
    http::{header, Method},
    middleware::{Compress, Logger},
    App,
    HttpServer,
};

use crate::states::AppState;

pub struct Server {
    host: String,
    port: u16,
    state: web::Data<AppState>,
}
 
impl Server {
    pub fn new(config: &shared::AppConfig) -> AppResult<Self> {
        let state: web::Data<AppState> = web::Data::new(AppState::new());

        Ok(Self {
            host: config.server.host.clone(),
            port: config.server.port,
            state,
        })
    }

    pub async fn run(self) -> AppResult<()> {
        let bind_address = format!("{}:{}", self.host, self.port);

        let origins: Vec<&str> = vec!["*"];
        let headers: Vec<header::HeaderName> = vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
        ];
        let methods: Vec<Method> = vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ];

        // tracing::info!("Starting HTTP server on {}", bind_address);

        HttpServer::new(move || {
            App::new()
                // Application state
                .app_data(self.state.clone())

                // Middleware
                .wrap(TracingLogger::default())
                .wrap(Compress::default())
                .wrap(
                    Cors::default()
                        .allowed_headers(headers)
                        .allowed_methods(methods)
                        .allowed_origin_fn(move |origin, _req_head| {
                            origins.iter().any(|o| origin.as_bytes() == o.as_bytes())
                        })
                        .max_age(3600),
                )
                // Configure routes
                .configure(crate::http_server::configure_routes)
        })
        .bind(bind_address)?
        .run()
        .await
        .map_err(|e| AppError::ServerError(format!("HTTP server error: {}", e)))
    }
}   
