use crate::repository::chat_room_repo::ChatRoomRepository;

pub struct ChatRoomService {
    repository: ChatRoomRepository,
}

impl ChatRoomService {
    pub fn new() -> Self {
        let repository = ChatRoomRepository::new();
        ChatRoomService { repository }
    }

    pub async fn create_chat_room(&self, room_name: String, created_by: i32) -> Result<i32, String> {
        self.repository.create_chat_room(&room_name, created_by).await
    }

    pub async fn join_chat_room(&self, user_id: i32, chatroom_id: i32) -> Result<String, String> {
        // First check if the room exists
        if !self.repository.does_room_exist(chatroom_id).await? {
            return Err("Chat room not found".to_string());
        }
        
        // Get the room name
        let room_name = self.repository.get_room_name(chatroom_id).await?;
        
        // If room exists, proceed with joining and propagate any potential error
        self.repository.add_user_to_chat_room(user_id, chatroom_id).await?;
        
        Ok(room_name)
    }

    pub async fn leave_chat_room(&self, user_id: i32, chatroom_id: i32) -> Result<(), String> {
        self.repository.remove_user_from_chat_room(user_id, chatroom_id).await
    }
}
