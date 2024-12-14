use axum::{
    routing::{any, get, post},
    Router
};
use tokio::sync::broadcast;
use tower_http::{
    cors::{CorsLayer, Any},
    add_extension::AddExtensionLayer,
};
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};
use mysql_async::Pool;
use crate::database::initialize_database;
mod handlers;
use crate::handlers::chat_room_apis::*;
use crate::handlers::user_auth_apis::*;
use crate::handlers::websocket_handler::ws_handler;
mod services;
mod repository;
mod database;

pub type ChatChannels = Arc<Mutex<HashMap<i32, (broadcast::Sender<(String, SocketAddr)>, broadcast::Receiver<(String, SocketAddr)>)>>>;

#[derive(Clone)]
pub struct AppState {
    pub chat_channels: ChatChannels,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            chat_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tokio::main]
async fn main() {
     // initilize the connection
     //url format: mysql://username:password@localhost/database_name
     //remember you also need to modify it at repository layer
     let database_url = "mysql://root:lyy@localhost/chat_app";
     let pool = Pool::new(database_url);
 
     // initilize the database
     if let Err(e) = initialize_database(&pool).await {
         eprintln!("Failed to initialize database: {}", e);
         return;
     }
    // Build the application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/api/chatrooms", post(create_chat_room))
        .route("/api/chatrooms/join", post(join_chat_room))
        .route("/api/chatrooms/leave", post(leave_chat_room))
        .route("/api/user/signup", post(user_signup))
        .route("/api/user/login", post(user_login))
        .route("/api/user/logout", post(user_logout))
        .route("/api/user/fetch_status", post(fetch_user_status))
        .route("/ws/{chat}", any(ws_handler))
        .layer(AddExtensionLayer::new(AppState::new()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();

}

// Root handler
async fn root() -> &'static str {
    "Welcome to the Rust server!"
}


