use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct WsQuery {
    user_id: i32,
    username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ChatMessage {
    pub user_id: i32,            
    pub username: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    // pub addr: SocketAddr,
}
