use mysql_async::prelude::Queryable; 
use mysql_async::{Pool, Conn};

pub async fn initialize_database(pool: &Pool) -> Result<(), String> {
    let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;

    // delete current exist table
    // drop_tables(&mut conn).await?;

    // create a new one
    create_users_table(&mut conn).await?;
    create_chatrooms_table(&mut conn).await?;
    create_user_in_chatroom_table(&mut conn).await?;
    create_messages_table(&mut conn).await?;

    Ok(())
}
// delete table
#[allow(dead_code)]
async fn drop_tables(conn: &mut Conn) -> Result<(), String> {
    // check the dependency
    conn.query_drop("DROP TABLE IF EXISTS UserInChatRoom")
        .await
        .map_err(|e| e.to_string())?;
    conn.query_drop("DROP TABLE IF EXISTS Messages")
        .await
        .map_err(|e| e.to_string())?;
    conn.query_drop("DROP TABLE IF EXISTS ChatRooms")
        .await
        .map_err(|e| e.to_string())?;
    conn.query_drop("DROP TABLE IF EXISTS Users")
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// create Users table
async fn create_users_table(conn: &mut Conn) -> Result<(), String> {
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Users (
            user_id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(50) UNIQUE NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            status ENUM('online', 'offline') DEFAULT 'offline',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .await
    .map_err(|e| e.to_string())
}

// create ChatRooms table
async fn create_chatrooms_table(conn: &mut Conn) -> Result<(), String> {
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS ChatRooms (
            chatroom_id INT AUTO_INCREMENT PRIMARY KEY,
            room_name VARCHAR(100) UNIQUE NOT NULL,
            created_by INT DEFAULT NULL, 
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (created_by) REFERENCES Users(user_id)
                ON DELETE SET NULL
        )",
    )
    .await
    .map_err(|e| e.to_string())
}


// create UserInChatRoom table
async fn create_user_in_chatroom_table(conn: &mut Conn) -> Result<(), String> {
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS UserInChatRoom (
            user_id INT NOT NULL,
            chatroom_id INT NOT NULL,
            joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (user_id, chatroom_id),
            FOREIGN KEY (user_id) REFERENCES Users(user_id)
                ON DELETE CASCADE,
            FOREIGN KEY (chatroom_id) REFERENCES ChatRooms(chatroom_id)
                ON DELETE CASCADE
        )"
    )
    .await
    .map_err(|e| e.to_string())
}

// create Messages table
async fn create_messages_table(conn: &mut Conn) -> Result<(), String> {
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Messages (
            message_id INT AUTO_INCREMENT PRIMARY KEY,
            chatroom_id INT NOT NULL,
            sender_id INT DEFAULT NULL, 
            message_text TEXT NOT NULL,
            sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (chatroom_id) REFERENCES ChatRooms(chatroom_id)
                ON DELETE CASCADE,
            FOREIGN KEY (sender_id) REFERENCES Users(user_id)
                ON DELETE SET NULL
        )",
    )
    .await
    .map_err(|e| e.to_string())
}

