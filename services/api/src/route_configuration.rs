use actix_web::web;
use presentation::routes::*;

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(health::routes);

    // API v1 routes
    cfg.service(
        web::scope("/api/v1").configure(user::configure), // .configure(tenant::configure)
    );
}
