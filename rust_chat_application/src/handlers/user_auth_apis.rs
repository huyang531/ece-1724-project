use axum::{extract::Query, extract::Json, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupPayload {
    username: String,
    email: String,
    password: String,
}


#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LogoutPayload {
    user_id: i32,
}

#[derive(Deserialize)]
pub struct FetchOnlineStatusQuery {
    room_id: i32,
}

pub async fn user_signup(Json(payload): Json<SignupPayload>) -> impl IntoResponse {
    // Logic to handle user signup goes here
}




pub async fn user_login(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    // Logic to handle user login goes here
}


pub async fn user_logout(Json(payload): Json<LogoutPayload>) -> impl IntoResponse {
    // Logic to handle user logout goes here
}


pub async fn fetch_user_online_status(Query(params): Query<FetchOnlineStatusQuery>) -> impl IntoResponse {
    // Logic to fetch user online status goes here
}

