use mysql_async::Pool;
use tokio::sync::OnceCell;
use mysql_async::prelude::*;
use mysql_async::params;

// 全局唯一的连接池
static DB_POOL: OnceCell<Pool> = OnceCell::const_new();

async fn get_db_pool() -> &'static Pool {
    DB_POOL
        .get_or_init(|| async {
            let database_url = "mysql://chat_user:password@localhost/chat_app";
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
            r"INSERT IGNORE INTO User(user_name, email, password_hash, status, created_at)
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
        let result: Option<(String,)> = conn
            .exec_first(
                r"SELECT user_name FROM User WHERE email = :email",
                params! {
                    "email" => email,
                },
            )
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn user_query(
        &self,
        email: &str,
        password_hash: &str,
    ) -> Result<(), String> {
        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
        let result: Option<(String,)> = conn
            .exec_first(
                r"SELECT user_id, user_name FROM User WHERE email = :email AND password_hash = :password_hash",
                params! {
                    "email" => email,
                    "password_hash" => password_hash,
                },
            )
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn user_login(&self, user_id: i32) -> Result<(), String> {
        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
        conn.exec_drop(
            r"UPDATE User SET status = 'online' WHERE user_id = :user_id",
            params! {
                "user_id" => user_id,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn user_logout(&self, user_id: i32) -> Result<(), String> {
        let pool = get_db_pool().await;
        let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
        conn.exec_drop(
            r"UPDATE User SET status = 'offline' WHERE user_id = :user_id",
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

      let user_id_list = conn
          .exec(
              r"SELECT user_id FROM user_in_chatroom WHERE room_id = :room_id",
              params! { "room_id" => room_id },
          )
          .await
          .map_err(|e| e.to_string())?
          .into_iter()
          .map(|row: mysql_async::Row| {
              row.get(0).ok_or_else(|| "Failed to get user_id".to_string())
          })
          .collect::<Result<Vec<i32>, String>>()?; // Collect into Result<Vec<i32>, String>

      Ok(user_id_list)
  }

   pub async fn fetch_user_status(&self, user_ids: Vec<i32>) -> Result<Vec<(i32, String)>, String> {
       let pool = get_db_pool().await;
       let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;

       let placeholders = user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
       let query = format!(
           "SELECT user_id, status FROM User WHERE user_id IN ({})",
           placeholders
       );

       let user_list = conn.exec(
           &query,
           user_ids.clone()
       )
       .await
       .map_err(|e| e.to_string())?
       .into_iter()
       .map(|row: mysql_async::Row| {
           let user_id: i32 = row.get(0).unwrap();
           let status: String = row.get(1).unwrap_or_default();
           (user_id, status)
       })
       .collect::<Vec<(i32, String)>>();

       Ok(user_list)
   }




}

