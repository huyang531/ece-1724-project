use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use crate::services::chat_room_service::ChatRoomService;

#[derive(Deserialize)]
pub struct CreateChatRoomPayload {
    pub room_name: String,
    pub user_id: i32
}

#[derive(Deserialize)]
pub struct JoinChatRoomPayload {
    pub user_id: i32,
    pub room_id: i32,
}

#[derive(Deserialize)]
pub struct LeaveChatRoomPayload {
    pub user_id: i32,
    pub room_id: i32,
}

pub async fn create_chat_room(
    Json(payload): Json<CreateChatRoomPayload>,
) -> impl IntoResponse {
    let service = ChatRoomService::new();
    match service.create_chat_room(payload.room_name, payload.user_id).await {
        Ok(room_id) => (StatusCode::CREATED, Json(json!({
            "message": "Chat room created",
            "room_id": room_id
        }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))),
    }
}

pub async fn join_chat_room(
    Json(payload): Json<JoinChatRoomPayload>,
) -> impl IntoResponse {
    let service = ChatRoomService::new();
    match service.join_chat_room(payload.user_id, payload.room_id).await {
        Ok(room_name) => (StatusCode::OK, Json(json!({
            "message": "Joined chat room",
            "room_name": room_name
        }))),
        Err(e) => match e.as_str() {
            "Chat room not found" => (StatusCode::NOT_FOUND, Json(json!({"error": e}))),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))),
        },
    }
}


pub async fn leave_chat_room(
    Json(payload): Json<LeaveChatRoomPayload>,
) -> impl IntoResponse {
    let service = ChatRoomService::new();
    match service.leave_chat_room(payload.user_id, payload.room_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Left chat room"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))),
    }
}
