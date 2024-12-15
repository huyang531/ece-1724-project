use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, Query}, response::IntoResponse, routing::any, Extension, Router
};
use axum_extra::TypedHeader;
use tokio::sync::{broadcast, Mutex};

use std::{borrow::Cow, sync::Arc};
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use mysql_async::{Pool, prelude::*};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

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
// async fn handle_socket(socket: WebSocket, who: SocketAddr, chat: i32, username: String, user_id: i32, state: Arc<AppState>) {
    // tracing::info!("Websocket context {who} created (user_id: {user_id})");
async fn handle_socket(socket: WebSocket, who: SocketAddr, chat: i32,state: Arc<AppState>, user_id: i32, username: String) {
    tracing::info!("Websocket context {who} created");
    // let username: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    // let username_clone = username.clone();
    // let user_id: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(None));
    // let user_id_clone = user_id.clone();

    let (mut sender, mut receiver) = socket.split();

    let mut rx = {
        let mut chat_channels = state.chat_channels.lock().await;
        chat_channels.entry(chat).or_insert_with(|| broadcast::channel(16)).0.subscribe()
    };

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        // let user_id = user_id_clone;
        let mut cnt = 0;
        loop {
            let msg = rx.recv().await.unwrap();
            // if msg.user_id == user_id.lock().await.unwrap_or_else(|| -1) {
            //     continue;
            // }
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
    // let user_id_clone = user_id.clone();
    let mut recv_task = tokio::spawn(async move {
        tracing::info!("Receive task created for {who} (user_id: {user_id})");
        let state = state_clone;
        // let username = username_clone;
        // let user_id = user_id_clone;
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

                    // let mut username = username.lock().await;
                    // let mut user_id = user_id.lock().await;
                    // if username.is_none() || user_id.is_none() {
                    //     *username = Some(msg.username.clone());
                    //     *user_id = Some(msg.user_id);
                    // }

                    

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
                            // let mut chat_channels = state.chat_channels.lock().await;
                            // let tx = &chat_channels.get_mut(&chat).unwrap().0;
                            // tx.send(msg).unwrap();
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
        // let username = username.lock().await;
        // let user_id = user_id.lock().await;
        tx.send(ChatMessage {
            user_id: -1,
            username: String::from("Server"),
            content: format!("User {} (user_id: {}) joined the chat room", username, user_id),
            // content: format!("User {} (user_id: {}) joined the chat room", (*username).clone().unwrap_or_else(|| String::from("Unknown")), user_id.unwrap_or_else(|| -1)),
            timestamp: chrono::Utc::now(),
            // addr: who,
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
        // let username = username.lock().await;
        // let user_id = user_id.lock().await;
        tx.send(ChatMessage {
            user_id: -1,
            username: String::from("Server"),
            content: format!("User {} (user_id: {}) left the chat room", username, user_id),
            // content: format!("User {} (user_id: {}) left the chat room", (*username).clone().unwrap_or_else(|| String::from("Unknown")), user_id.unwrap_or_else(|| -1)),
            timestamp: chrono::Utc::now(),
            // addr: who,
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


    // returning from the handler closes the websocket connection
    tracing::info!("Websocket context {who} destroyed (user_id: {})", user_id);
    // tracing::info!("Websocket context {who} destroyed (user_id: {})", user_id.lock().await.unwrap_or_else(|| -1));
}

/// Helper function to send a message to a channel (chat)
async fn send_to_channel(chat: i32, state: Arc<AppState>, msg: ChatMessage) {
    let mut chat_channels = state.chat_channels.lock().await;
    let tx = &chat_channels.get_mut(&chat).unwrap().0;
    tx.send(msg).unwrap();
}

