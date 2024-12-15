use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub struct ChatMessage {
    pub user_id: i32,            
    pub username: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    // pub addr: SocketAddr,
}
