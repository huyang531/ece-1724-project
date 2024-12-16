use mysql_async::Pool;
use tokio::sync::OnceCell;
use mysql_async::prelude::*;
use mysql_async::params;

// Static instance
static DB_POOL: OnceCell<Pool> = OnceCell::const_new();

// Async function call to get database pool
async fn get_db_pool() -> &'static Pool {
    DB_POOL
        .get_or_init(|| async {
            let database_url = "mysql://root:root@localhost/chat_app";
            Pool::new(database_url)
        })
        .await
}

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository
    }

    pub async fn user_sign_up(
        &self,
        email: &str,
        user_name: &str,
        password_hash: &str,
        created_at: i64,
    ) -> Result<(), String> {
        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
        
        conn.exec_drop(
            r"INSERT IGNORE INTO Users(username, email, password_hash, status, created_at)
              VALUES(:user_name, :email, :password_hash, 'offline', :created_at)",
            params! {
                "user_name" => user_name,
                "email" => email,
                "password_hash" => password_hash,
                "created_at" => created_at,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub async fn user_check_exist(&self, email: String) -> Result<(), String> {
        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
    
        let result: Option<(i32,)> = conn
            .exec_first(
                r"SELECT user_id FROM Users WHERE email = :email",
                params! { "email" => email },
            )
            .await
            .map_err(|e| e.to_string())?;
    
        match result {
            Some(_) => Err("User exist".to_string()), // If user exists, return Ok(())
            None => Ok(()), // If no result, return an error
        }
    }

    pub async fn user_query(
        &self,
        email: &str,
        password_hash: &str,
    ) -> Result<Option<(i32, String)>, String> {
        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
        
        let result: Option<(i32, String)> = conn
            .exec_first(
                r"SELECT user_id, username FROM Users 
                 WHERE email = :email AND password_hash = :password_hash",
                params! {
                    "email" => email,
                    "password_hash" => password_hash,
                },
            )
            .await
            .map_err(|e| e.to_string())?;
    
        match result {
            Some((user_id, username)) => {
                // update user status to online
                conn.exec_drop(
                    r"UPDATE Users SET status = 'online'
                     WHERE user_id = :user_id",
                    params! {
                        "user_id" => user_id,
                    },
                )
                .await
                .map_err(|e| e.to_string())?;
                
                // return user_id and username
                Ok(Some((user_id, username)))
            },
            None => Ok(None),
        }
    }

    pub async fn user_logout(&self, user_id: i32) -> Result<(), String> {
        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
        
        conn.exec_drop(
            r"UPDATE Users SET status = 'offline' WHERE user_id = :user_id",
            params! {
                "user_id" => user_id,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub async fn fetch_user_list(&self, room_id: i32) -> Result<Vec<i32>, String> {
        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;

        conn.exec(
            r"SELECT user_id FROM UserInChatRoom WHERE chatroom_id = :room_id",
            params! { "room_id" => room_id },
        )
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|row: mysql_async::Row| row.get(0).ok_or_else(|| "Failed to get user_id".to_string()))
        .collect()
    }

    pub async fn fetch_user_status(&self, user_ids: Vec<i32>) -> Result<Vec<(i32, String)>, String> {
        if user_ids.is_empty() {
            return Ok(Vec::new());
        }

        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;

        let placeholders = user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!(
            "SELECT user_id, status FROM Users WHERE user_id IN ({})",
            placeholders
        );

        conn.exec(&query, user_ids.clone())
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|row: mysql_async::Row| {
                let user_id: i32 = row.get(0)
                    .ok_or_else(|| "Failed to extract user_id".to_string())?;
                let status: String = row.get(1)
                    .unwrap_or_else(|| "offline".to_string());
                Ok((user_id, status))
            })
            .collect::<Result<Vec<(i32, String)>, String>>()
    }
}
