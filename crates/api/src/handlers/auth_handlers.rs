use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{
    db::user_repo::{create_user, get_user_by_email},
    types::auth_types::{LoginPayload, RegisterPayload},
    utils::jwt::create_token,
};

pub async fn login(payload: web::Json<LoginPayload>, pool: web::Data<PgPool>) -> impl Responder {
    let payload = payload.into_inner();

    match get_user_by_email(&pool, &payload.email).await {
        Ok(Some(user)) => {
            if user.password_hash != payload.password {
                return HttpResponse::Unauthorized().json("Invalid credentials");
            }

            let token = match create_token(&user.id.to_string()) {
                Ok(token) => token,
                Err(e) => {
                    println!("JWT error: {:?}", e);
                    return HttpResponse::InternalServerError().json("Token generation failed");
                }
            };

            HttpResponse::Ok().json(serde_json::json!({
                "token": token,
                "user": {
                    "id": user.id,
                    "email": user.email,
                    "username": user.username
                }
            }))
        }

        Ok(None) => HttpResponse::Unauthorized().json("Invalid credentials"),

        Err(e) => {
            println!("DB error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn register(
    payload: web::Json<RegisterPayload>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let payload = payload.into_inner();

    match get_user_by_email(&pool, &payload.email).await {
        Ok(_) => {
            return HttpResponse::BadRequest().json("User already exists");
        }
        Err(_) => {
            // continue
        }
    }

    match create_user(&pool, &payload).await {
        Ok(user) => {
            let token = create_token(&user.id.to_string()).unwrap();

            HttpResponse::Ok().json(serde_json::json!({
                "token": token,
                "user": {
                    "id": user.id,
                    "email": user.email,
                    "username": user.username
                }
            }))
        }
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create user")
        }
    }
}

pub async fn refresh() -> impl Responder {
    HttpResponse::Ok().body("refresh")
}
