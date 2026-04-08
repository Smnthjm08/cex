use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password_hash: String,
}
