use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub message: String,
    pub uid: i32,
    pub username: String,
}

#[derive(Serialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupResponse {
    pub user_id: i32,
    pub username: String,
    pub status: String,
}
