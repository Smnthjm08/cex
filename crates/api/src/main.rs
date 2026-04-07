use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod db;
mod handlers;
mod models;
mod routes;
mod types;

use crate::db::connection::connect;
use crate::routes::{auth_routes::auth_routes, orders_routes::orders_routes};

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = connect(&database_url)
        .await
        .expect("error connecting to db!");

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .service(
                web::scope("/api/v1")
                    .service(auth_routes())
                    .service(orders_routes()),
            )
            .default_service(web::to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
