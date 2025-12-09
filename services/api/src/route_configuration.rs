use presentation::routes::*;

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(health::routes);
    // .configure(user::routes)
    // .configure(tenant::routes);
}
