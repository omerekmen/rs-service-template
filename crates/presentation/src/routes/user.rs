use actix_web::web;

use crate::handlers::user_handlers;

/// Configure user routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(user_handlers::create_user))
            .route("", web::get().to(user_handlers::list_users))
            .route("/{id}", web::get().to(user_handlers::get_user))
            .route("/{id}", web::put().to(user_handlers::update_user))
            .route("/{id}", web::delete().to(user_handlers::delete_user))
            .route(
                "/username/{username}",
                web::get().to(user_handlers::get_user_by_username),
            ),
    );
}
