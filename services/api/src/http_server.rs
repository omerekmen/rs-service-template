use actix_cors::Cors;
use actix_web::{
    web,
    http::{header, Method},
    middleware::{Compress, Logger},
    App,
    HttpServer,
};

use crate::route_configuration::configure_routes;
use presentation::states::AppState;

pub struct Server {
    host: String,
    port: u16,
    state: web::Data<AppState>,
    origins: Vec<String>,
    headers: Vec<header::HeaderName>,
    methods: Vec<Method>
}

impl Server {
    pub async fn new(config: &shared::AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let app_state = AppState::new().load(config).await?;
        let state: web::Data<AppState> = web::Data::new(app_state);

        let origins: Vec<String> = vec!["*".to_string()];
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

        Ok(Self {
            host: config.server.host.clone(),
            port: config.server.port,
            state,
            origins,
            headers,
            methods,
        })
    }
    
    pub async fn run(self) -> Result<(), actix_web::Error> {
        let bind_address = format!("{}:{}", self.host, self.port);
        
        let headers = self.headers.clone();
        let methods = self.methods.clone();
        let origins = self.origins.clone();
        let shared_state = self.state.clone();

        tracing::info!("Starting HTTP server on {}", bind_address);

        HttpServer::new(move || {
            let mut cors = Cors::default()
                .allowed_headers(headers.clone())
                .allowed_methods(methods.clone())
                .max_age(3600);

            if origins.len() == 1 && origins[0] == "*" {
                cors = cors.allow_any_origin();
            } else {
                for origin in &origins {
                    cors = cors.allowed_origin(origin);
                }
            }

            App::new()
                .app_data(shared_state.clone())
                // .wrap(TrackingLogger::default)
                .wrap(Logger::default())
                .wrap(Compress::default())
                .wrap(cors)
                .configure(configure_routes)
        })
            .bind(bind_address)?
            .run()
            .await
            .map_err(actix_web::Error::from)
    }


    pub fn set_origins(&mut self, origins: Vec<&str>) {
        self.origins = origins.into_iter().map(|s| s.to_string()).collect();
    }

    pub fn set_headers(&mut self, headers: Vec<header::HeaderName>){
        self.headers = headers;
    }

    pub fn set_methods(&mut self, methods: Vec<Method>){
        self.methods = methods;
    }

}   
