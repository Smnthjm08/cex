use crate::handlers::auth_handlers;
use actix_web::{web, Scope};

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(auth_handlers::login))
        .route("/register", web::post().to(auth_handlers::register))
        .route("/refresh", web::get().to(auth_handlers::refresh))
}
