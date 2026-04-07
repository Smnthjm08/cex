use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{db::user_repo::{create_user, get_user_by_email}, models::user::CreateUserPayload, types::auth_types::LoginPayload};

pub async fn login(payload: web::Json<LoginPayload>,
    pool: web::Data<PgPool>,) -> impl Responder {
let payload = payload.into_inner();
let res = get_user_by_email(&pool, &payload.email).await.unwrap();

    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create user")
        }
    }}

pub async fn register(
    payload: web::Json<CreateUserPayload>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let res = create_user(&pool, &payload).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("register"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create user")
        }
    }
}

pub async fn refresh() -> impl Responder {
    HttpResponse::Ok().body("refresh")
}
