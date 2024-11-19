use axum::{extract::Json, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateChatRoomPayload {
    room_name: String,
}

#[derive(Deserialize)]
pub struct JoinChatRoomPayload {
    user_id: i32,
    room_id: i32,
}

#[derive(Deserialize)]
pub struct LeaveChatRoomPayload {
    user_id: i32,
    room_id: i32,
}

pub async fn create_chat_room(Json(payload): Json<CreateChatRoomPayload>) -> impl IntoResponse {
    // Logic to handle creating a chat room goes here
}

pub async fn join_chat_room(Json(payload): Json<JoinChatRoomPayload>) -> impl IntoResponse {
    // Logic to handle joining a chat room goes here
}

pub async fn leave_chat_room(Json(payload): Json<LeaveChatRoomPayload>) -> impl IntoResponse {
    // Logic to handle leaving a chat room goes here
}
