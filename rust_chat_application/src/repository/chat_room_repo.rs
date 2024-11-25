use mysql_async::{Pool, prelude::*};

pub struct ChatRoomRepository {
    pool: Pool,
}

impl ChatRoomRepository {
    pub fn new() -> Self {
        let database_url = "mysql://chat_user:password@localhost/chat_app";
        let pool = Pool::new(database_url);
        ChatRoomRepository { pool }
    }

    pub async fn create_chat_room(&self, room_name: &str, created_by: i32) -> Result<(), String> {
        let mut conn = self.pool.get_conn().await.map_err(|e| e.to_string())?;
        conn.exec_drop(
            r"INSERT IGNORE INTO ChatRooms (room_name, created_by) VALUES (:room_name, :created_by)",
            params! {
                "room_name" => room_name,
                "created_by" => created_by,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
    

    pub async fn add_user_to_chat_room(&self, user_id: i32, chatroom_id: i32) -> Result<(), String> {
        let mut conn = self.pool.get_conn().await.map_err(|e| e.to_string())?;
        conn.exec_drop(
            r"INSERT IGNORE INTO UserInChatRoom (user_id, chatroom_id) VALUES (:user_id, :chatroom_id)",
            params! {
                "user_id" => user_id,
                "chatroom_id" => chatroom_id,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
    

    pub async fn remove_user_from_chat_room(&self, user_id: i32, chatroom_id: i32) -> Result<(), String> {
        let mut conn = self.pool.get_conn().await.map_err(|e| e.to_string())?;
        conn.exec_drop(
            r"DELETE FROM UserInChatRoom WHERE user_id = :user_id AND chatroom_id = :chatroom_id",
            params! {
                "user_id" => user_id,
                "chatroom_id" => chatroom_id,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
