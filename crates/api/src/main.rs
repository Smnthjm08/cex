use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod handlers;
mod routes;

use crate::routes::{auth_routes::auth_routes, orders_routes::orders_routes};

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(auth_routes())
                    .service(orders_routes())
            )
            .default_service(web::to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
