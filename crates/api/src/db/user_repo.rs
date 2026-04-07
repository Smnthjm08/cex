use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::user::User, types::auth_types::RegisterPayload};

pub async fn create_user(pool: &PgPool, user: &RegisterPayload) -> Result<User, sqlx::Error> {
    let uuid = Uuid::new_v4();

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, username, first_name, last_name, password_hash)
         VALUES ($1,$2,$3,$4,$5,$6)
         RETURNING id, email, username, first_name, last_name, password_hash, created_at, updated_at",
        uuid,
        user.email,
        user.username,
        user.first_name,
        user.last_name,
        user.password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, username, first_name, last_name, password_hash, created_at, updated_at 
         FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
