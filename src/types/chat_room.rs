use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateChatRoomRequest {
    pub user_id: i32,
    pub room_name: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CreateChatRoomResponse {
    pub message: String,
    pub room_id: i32,
}

#[derive(Serialize)]
pub struct JoinChatRoomRequest {
    pub user_id: i32,
    pub room_id: i32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct JoinChatRoomResponse {
    pub message: String,
    pub room_name: String,
}

