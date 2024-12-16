use axum::{extract::Json, response::IntoResponse};
use serde::Deserialize;
use lazy_static::lazy_static;
use serde_json::json;
use tokio::sync::Mutex;
use chrono::Utc;
use crate::services::user_auth_service::UserAuthService;


#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    pub email: String,
    pub password: String,
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

lazy_static! {
    static ref USERSERVICE: Mutex<UserAuthService> = Mutex::new(UserAuthService::new());
}


use axum::http::StatusCode;

pub async fn user_signup(Json(payload): Json<SignupPayload>) -> impl IntoResponse {
    let service = USERSERVICE.lock().await;
    match service.user_check_exist(payload.email.clone()).await {
        Err(_e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "this email has existed"})),
        ),
        Ok(_) => {
            match service
                .user_sign_up(
                    payload.email,
                    payload.username,
                    payload.password,
                    Utc::now().timestamp(),
                )
                .await
            {
                Ok(_) => (
                    StatusCode::OK,
                    Json(json!({"message": "User signed up successfully"})),
                ),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e})),
                ),
            }
        }
    }
}


pub async fn user_login(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    let service = USERSERVICE.lock().await;

    // call user_query() to fetch username and id
    match service.user_query(payload.email, payload.password).await {
        Ok(Some((uid, username))) => (
            StatusCode::OK,
            Json(json!({
                "message": "User logged in successfully",
                "uid": uid,
                "username": username
            })),
        ),
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e})),
        ),
    }
}

pub async fn user_logout(Json(payload): Json<LogoutPayload>) -> impl IntoResponse {
    let service = USERSERVICE.lock().await;
    match service.user_logout(payload.user_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"message": "User logged out successfully"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e})),
        ),
    }
}

pub async fn fetch_user_status(Json(params): Json<FetchOnlineStatusQuery>) -> impl IntoResponse {
    let service = USERSERVICE.lock().await;

    // Fetch the list of users in the specified room
    match service.fetch_user_list(params.room_id).await {
        Ok(user_list) => {
            // Fetch the status of the users
            match service.fetch_user_status(user_list).await {
                Ok(online_users) => {
                    // Map the online users to a response-friendly format
                    let online_users_json = online_users.into_iter().map(|(user_id, status)| {
                        json!({
                            "user_id": user_id,
                            "status": status,
                        })
                    }).collect::<Vec<_>>();

                    // Return the response with the list of online users
                    (StatusCode::OK, Json(json!({ "online_users": online_users_json })))
                },
                Err(e) => {
                    // If fetching user statuses fails, return an error
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e })))
                },
            }
        },
        Err(e) => {
            // If fetching the user list fails, return an error
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e })))
        },
    }
}
