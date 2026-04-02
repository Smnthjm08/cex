use actix_web::{web, Scope};

use crate::handlers::orders_handlers::{depth, list_orders};

pub fn orders_routes()-> Scope {
    web::scope("/orders")
        .route("/depth", web::get().to(depth))
        .route("/", web::get().to(list_orders))
}