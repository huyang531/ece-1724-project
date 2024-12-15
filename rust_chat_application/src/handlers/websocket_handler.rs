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
    // Query(query): Query<WsQuery>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    println!("Incoming websocket connection from {addr}");

    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, chat, state))
    // ws.on_upgrade(move |socket| handle_socket(socket, addr, chat, Arc::new(AppState::new())))
}

/// Actual websocket statemachine (one will be spawned per connection)
// async fn handle_socket(socket: WebSocket, who: SocketAddr, chat: i32, username: String, user_id: i32, state: Arc<AppState>) {
    // println!("Websocket context {who} created (user_id: {user_id})");
async fn handle_socket(socket: WebSocket, who: SocketAddr, chat: i32,state: Arc<AppState>) {
    println!("Websocket context {who} created");
    let username: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let username_clone = username.clone();
    let user_id: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(None));
    let user_id_clone = user_id.clone();

    let (mut sender, mut receiver) = socket.split();

    let mut rx = {
        let mut chat_channels = state.chat_channels.lock().await;
        chat_channels.entry(chat).or_insert_with(|| broadcast::channel(16)).0.subscribe()
    };

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let user_id = user_id_clone;
        let mut cnt = 0;
        loop {
            let msg = rx.recv().await.unwrap();
            // if msg.user_id == user_id.lock().await.unwrap_or_else(|| -1) {
            //     continue;
            // }
            cnt += 1;
            if sender.send(Message::Text(serde_json::to_string(&msg).unwrap())).await.is_err() {
                println!("client {who} abruptly disconnected because we could not send message to it");
                break;
            }
        }
        cnt
    });

    // This second task will receive messages from client and print them on server console
    let state_clone = state.clone();
    let user_id_clone = user_id.clone();
    let mut recv_task = tokio::spawn(async move {
        let state = state_clone;
        let username = username_clone;
        let user_id = user_id_clone;
        let mut cnt = 0;
        loop {
            let msg = receiver.next().await.unwrap();
            match msg {
                Ok(Message::Close(_)) => {
                    println!("Client {who} sent close message");
                    break;
                }
                Ok(Message::Text(msg)) => {
                    cnt += 1;
                    // Deserialize the message and send it to the chat channel
                    let msg: ChatMessage = serde_json::from_str(&msg).unwrap();

                    let mut username = username.lock().await;
                    let mut user_id = user_id.lock().await;
                    if username.is_none() || user_id.is_none() {
                        *username = Some(msg.username.clone());
                        *user_id = Some(msg.user_id);
                    }

                    // Add it to db
                    let mut conn = state.pool.get_conn().await.unwrap();
                    conn.exec_drop(
                        r"INSERT INTO ChatMessages (chatroom_id, sender_id, message_text, sent_at) VALUES (:chatroom_id, :sender_id, :message_text, :sent_at)",
                        params! {
                            "chatroom_id" => chat,
                            "sender_id" => msg.user_id,
                            "message_text" => msg.content.clone(),
                            "sent_at" => msg.timestamp.to_string(),
                        },
                    ).await
                    .expect("Could not insert message into db");

                    let mut chat_channels = state.chat_channels.lock().await;
                    let tx = &chat_channels.get_mut(&chat).unwrap().0;
                    tx.send(msg).unwrap();
                }
                Err(e) => {
                    println!("Client {who} abruptly disconnected due to {e}");
                    break;
                },
                _ => {
                    println!("Client {who} sent a message of a type we do not support");
                    break;
                },
            }
        }
        cnt
    });

    // Join chat room
    {
        let mut chat_channels = state.chat_channels.lock().await;
        let tx = &chat_channels.get_mut(&chat).unwrap().0;
        let username = username.lock().await;
        let user_id = user_id.lock().await;
        tx.send(ChatMessage {
            user_id: -1,
            username: String::from("Server"),
            content: format!("User {} (user_id: {}) joined the chat room", (*username).clone().unwrap_or_else(|| String::from("Unknown")), user_id.unwrap_or_else(|| -1)),
            timestamp: chrono::Utc::now(),
            // addr: who,
        }).unwrap();
    }

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => println!("{a} messages sent to {who}"),
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {b} messages"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // Leave chat room
    {
        let mut chat_channels = state.chat_channels.lock().await;
        let tx = &chat_channels.get_mut(&chat).unwrap().0;
        let username = username.lock().await;
        let user_id = user_id.lock().await;
        tx.send(ChatMessage {
            user_id: -1,
            username: String::from("Server"),
            content: format!("User {} (user_id: {}) left the chat room", (*username).clone().unwrap_or_else(|| String::from("Unknown")), user_id.unwrap_or_else(|| -1)),
            timestamp: chrono::Utc::now(),
            // addr: who,
        }).unwrap();

        // Call leave chat room api
        let mut conn = state.pool.get_conn().await.unwrap();
        conn.exec_drop(
            r"DELETE FROM UserInChatRoom WHERE user_id = :user_id AND chatroom_id = :chatroom_id",
            params! {
                "user_id" => *user_id,
                "chatroom_id" => chat,
            },
        ).await
        .expect("Could not leave chat room");
    }


    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed (user_id: {})", user_id.lock().await.unwrap_or_else(|| -1));
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: &Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
