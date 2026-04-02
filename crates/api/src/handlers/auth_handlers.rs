use actix_web::{HttpResponse, Responder};

pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("login")
}

pub async fn register() -> impl Responder {
    HttpResponse::Ok().body("register")
}

pub async fn refresh() -> impl Responder {
    HttpResponse::Ok().body("refresh")
}