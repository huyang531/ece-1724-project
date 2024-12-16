use axum::{
    routing::{any, get, post}, Extension, Router
};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};
use tower_http::{
    add_extension::AddExtensionLayer, cors::{Any, CorsLayer}, trace::{DefaultMakeSpan, TraceLayer}
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::{collections::HashMap, fmt::Display, net::SocketAddr, sync::Arc};
use mysql_async::Pool;
use chrono::{DateTime, Utc};

use crate::database::initialize_database;
use crate::handlers::chat_room_apis::*;
use crate::handlers::user_auth_apis::*;
use crate::handlers::websocket_handler::ws_handler;

mod handlers;
mod services;
mod repository;
mod database;

#[derive(Deserialize)]
pub struct WsQuery {
    user_id: i32,
    username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub user_id: i32,            
    pub username: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    // pub addr: SocketAddr,
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}

pub type ChatChannels = Arc<Mutex<HashMap<i32, (broadcast::Sender<ChatMessage>, broadcast::Receiver<ChatMessage>)>>>;

#[derive(Clone)]
pub struct AppState {
    pub chat_channels: ChatChannels,
    pub pool: Pool,
    // pub usernames: Arc<Mutex<HashMap<SocketAddr, String>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            chat_channels: Arc::new(Mutex::new(HashMap::new())),
            pool: Pool::new("mysql://root:root@localhost/chat_app"),
            // usernames: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
//mysql://root:root@localhost/chat_app
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
     // initilize the connection
     //url format: mysql://username:password@localhost/database_name
     //remember you also need to modify it at repository layer
     let database_url = "mysql://root:root@localhost/chat_app";
     let pool = Pool::new(database_url);
 
     // initilize the database
     if let Err(e) = initialize_database(&pool).await {
         eprintln!("Failed to initialize database: {}", e);
         return;
     }
    // Build the application with routes
    let app = Router::new()
        // .route("/", get(root))
        .route("/api/chatrooms", post(create_chat_room))
        .route("/api/chatrooms/join", post(join_chat_room))
        .route("/api/chatrooms/leave", post(leave_chat_room))
        .route("/api/user/signup", post(user_signup))
        .route("/api/user/login", post(user_login))
        .route("/api/user/logout", post(user_logout))
        .route("/api/user/fetch_status", post(fetch_user_status))
        .route("/ws/{chat}", any(ws_handler))
        .layer(Extension(Arc::new(AppState::new())))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // let addr = SocketAddr::from(([0,0,0,0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();

}

// Root handler
async fn root() -> &'static str {
    "Welcome to the Rust server!"
}

pub fn init() -> tracing_appender::non_blocking::WorkerGuard {
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();

    guard
}
