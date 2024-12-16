use crate::repository::user_repo::UserRepository;
use lazy_static::lazy_static;

lazy_static! {
    static ref USER_AUTH_SERVICE: UserAuthService = UserAuthService::new();
}

pub struct UserAuthService {
    repository: UserRepository,
}


impl UserAuthService {
    pub fn new() -> Self {
        let repository = UserRepository::new();
        UserAuthService { repository }
    }

    pub async fn user_check_exist(&self, email: String) -> Result<(), String> {
        // Call the repository function that returns Result<Option<T>, String>
        match self.repository.user_check_exist(email).await {
            // If the user doesn't exist (Option is None)
            Ok(()) => Ok(()),
    
            // If there's an error from the repository (Err case)
            Err(e) => Err(e),
        }
    }

    pub async fn user_sign_up(
        &self,
        email: String,
        user_name: String,
        password_hash: String,
        created_at: i64,
    ) -> Result<(), String> {
        self.repository
            .user_sign_up(&email, &user_name, &password_hash, created_at)
            .await
    }

    pub async fn user_query(
        &self,
        email: String,
        password_hash: String,
    ) -> Result<Option<(i32, String)>, String> {
        self.repository.user_query(&email, &password_hash).await
    }

    pub async fn user_logout(&self, user_id: i32) -> Result<(), String> {
        self.repository.user_logout(user_id).await
    }


    pub async fn fetch_user_list(&self, room_id:i32) -> Result<Vec<i32>, String> {
        self.repository.fetch_user_list(room_id).await
    }

    pub async fn fetch_user_status(&self, user_id_list: Vec<i32>) -> Result<Vec<(i32,String)>, String> {
        self.repository.fetch_user_status(user_id_list).await
    }
}
