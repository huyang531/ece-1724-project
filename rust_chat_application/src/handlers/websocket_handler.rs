use std::sync::Arc;
use std::net::SocketAddr;

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, Query}, response::IntoResponse, Extension
};
use axum_extra::TypedHeader;
use tokio::sync::broadcast;
use mysql_async::{Pool, prelude::*, Row};
//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};
use chrono::{DateTime, Utc}; // Added DateTime and Utc

use crate::{AppState, ChatMessage, WsQuery};

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(chat): Path<i32>,
    Query(query): Query<WsQuery>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    tracing::info!("Incoming websocket connection from {addr}");

    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::info!("`{user_agent}` at {addr} connected.");

    
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, chat, state, query.user_id, query.username.clone()))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr, chat: i32,state: Arc<AppState>, user_id: i32, username: String) {
    tracing::info!("Websocket context {who} created");
    let (mut sender, mut receiver) = socket.split();

    // Fetch chat history before setting up the broadcast subscription
    match fetch_chat_history(&state.pool, chat).await {
        Ok(history) => {
            // Send chat history as messages
            for msg in history {
                if sender.send(Message::Text(serde_json::to_string(&msg).unwrap())).await.is_err() {
                    tracing::error!("Failed to send chat history to client {who}");
                    return;
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch chat history: {}", e);
            let err_msg = ChatMessage {
                user_id: -1,
                username: String::from("Server"),
                content: String::from("Failed to load chat history"),
                timestamp: chrono::Utc::now(),
            };
            if sender.send(Message::Text(serde_json::to_string(&err_msg).unwrap())).await.is_err() {
                return;
            }
        }
    }

    let mut rx = {
        let mut chat_channels = state.chat_channels.lock().await;
        chat_channels.entry(chat).or_insert_with(|| broadcast::channel(16)).0.subscribe()
    };

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let mut cnt = 0;
        loop {
            let msg = rx.recv().await.unwrap();
            cnt += 1;
            if sender.send(Message::Text(serde_json::to_string(&msg).unwrap())).await.is_err() {
                tracing::info!("client {who} abruptly disconnected because we could not send message to it");
                break;
            }
        }
        cnt
    });

    // This second task will receive messages from client and print them on server console
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        tracing::info!("Receive task created for {who} (user_id: {user_id})");
        let state = state_clone;
        let mut cnt = 0;
        loop {
            let msg = receiver.next().await.unwrap();
            tracing::info!("Received message from {who}");
            match msg {
                Ok(Message::Close(_)) => {
                    tracing::info!("Client {who} sent close message");
                    break;
                }
                Ok(Message::Text(msg)) => {
                    tracing::info!("Received message from {who}: {msg}");
                    cnt += 1;
                    // Deserialize the message and send it to the chat channel
                    let msg: ChatMessage = serde_json::from_str(&msg).unwrap();

                    // Add it to db
                    let mut conn = state.pool.get_conn().await.unwrap();
                    match conn.exec_drop(
                        r"INSERT INTO Messages (chatroom_id, sender_id, message_text, sent_at) VALUES (:chatroom_id, :sender_id, :message_text, :sent_at)",
                        params! {
                            "chatroom_id" => chat,
                            "sender_id" => msg.user_id,
                            "message_text" => msg.content.clone(),
                            "sent_at" => msg.timestamp.to_rfc3339(),
                        },
                    ).await{
                        Ok(_) => {
                            // Send to the channel
                            send_to_channel(chat, state.clone(), msg).await;
                            
                        },
                        Err(e) => {
                            tracing::error!("Could not insert message into db due to {e}");
                            let err_msg = ChatMessage {
                                user_id: -1,
                                username: String::from("Server"),
                                content: format!("New Message: {msg}. Could not insert message into db due to {e}"),
                                timestamp: chrono::Utc::now(),
                                // addr: who,
                            };
                            send_to_channel(chat, state.clone(), err_msg).await;
                            break;
                        }
                    }

                    
                }
                Err(e) => {
                    tracing::info!("Client {who} abruptly disconnected due to {e}");
                    break;
                },
                _ => {
                    tracing::info!("Client {who} sent a message of a type we do not support");
                    break;
                },
            }
        }
        tracing::info!("Receive task destroyed for {who} (user_id: {user_id})");
        cnt
    });

    // Join chat room
    {
        let mut chat_channels = state.chat_channels.lock().await;
        let tx = &chat_channels.get_mut(&chat).unwrap().0;
        tx.send(ChatMessage {
            user_id: -1,
            username: String::from("Server"),
            content: format!("User {} (user_id: {}) joined the chat room", username, user_id),
            timestamp: chrono::Utc::now(),
        }).unwrap();
    }

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => tracing::info!("{a} messages sent to {who}"),
                Err(a) => tracing::info!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => tracing::info!("Received {b} messages"),
                Err(b) => tracing::info!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // Leave chat room
    {
        let mut chat_channels = state.chat_channels.lock().await;
        let tx = &chat_channels.get_mut(&chat).unwrap().0;
        tx.send(ChatMessage {
            user_id: -1,
            username: String::from("Server"),
            content: format!("User {} (user_id: {}) left the chat room", username, user_id),
            timestamp: chrono::Utc::now(),
        }).unwrap();

        // Call leave chat room api
        let mut conn = state.pool.get_conn().await.unwrap();
        conn.exec_drop(
            r"DELETE FROM UserInChatRoom WHERE user_id = :user_id AND chatroom_id = :chatroom_id",
            params! {
                "user_id" => user_id,
                "chatroom_id" => chat,
            },
        ).await
        .expect("Could not leave chat room");
    }


    tracing::info!("Websocket context {who} destroyed (user_id: {})", user_id);
}

/// Helper function to send a message to a channel (chat)
async fn send_to_channel(chat: i32, state: Arc<AppState>, msg: ChatMessage) {
    let mut chat_channels = state.chat_channels.lock().await;
    let tx = &chat_channels.get_mut(&chat).unwrap().0;
    tx.send(msg).unwrap();
}

/// Helper function to fetch chat history from the database
async fn fetch_chat_history(pool: &Pool, chat_id: i32) -> Result<Vec<ChatMessage>, mysql_async::Error> {
    let mut conn = pool.get_conn().await?;
    
    let messages: Vec<ChatMessage> = conn
        .exec_map(
            r"SELECT m.sender_id, u.username, m.message_text, 
              UNIX_TIMESTAMP(m.sent_at) as sent_at 
              FROM Messages m 
              LEFT JOIN Users u ON m.sender_id = u.user_id 
              WHERE m.chatroom_id = :chat_id 
              ORDER BY m.sent_at ASC 
              LIMIT 50",
            params! {
                "chat_id" => chat_id,
            },
            |row: Row| {
                let timestamp_unix: i64 = row.get("sent_at").unwrap();
                let timestamp = DateTime::<Utc>::from_timestamp(timestamp_unix, 0).unwrap();
                
                ChatMessage {
                    user_id: row.get("sender_id").unwrap(),
                    username: row.get::<String, _>("username")
                               .unwrap_or_else(|| String::from("Deleted User")),
                    content: row.get("message_text").unwrap(),
                    timestamp,
                }
            }
        )
        .await?;

    Ok(messages)
}
