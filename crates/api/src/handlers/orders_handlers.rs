use actix_web::{HttpResponse, Responder};

pub async fn list_orders() -> impl Responder {
    HttpResponse::Ok().body("orders root")
}

pub async fn depth() -> impl Responder {
    HttpResponse::Ok().body("order depth")
}
