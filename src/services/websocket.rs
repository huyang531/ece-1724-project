use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use futures::{lock::Mutex, stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
// src/services/websocket.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use tokio_tungstenite_wasm::{connect, WebSocketStream, Message};
// use futures_util::stream::stream::StreamExt;
use yew::{html::IntoPropValue, platform::time::sleep, Callback};

use crate::types::chat::ChatMessage;

pub struct WebSocketService {
    sender: Arc<Mutex<Option<SplitSink<WebSocketStream, Message>>>>,
    _receiver: Arc<Mutex<Option<SplitStream<WebSocketStream>>>>,
    user_id: i32,
    username: String,
    cancel: Arc<AtomicBool>,
}

impl WebSocketService {
    pub fn new(url: &str, user_id: i32, username: String, on_message: Callback<ChatMessage>) -> Self {
        log::debug!("WebSocketService new() called");

        let sender = Arc::new(Mutex::new(None));
        let sender_clone = sender.clone();
        let receiver = Arc::new(Mutex::new(None));
        let receiver_clone = receiver.clone();
        let url_clone = url.to_string();
        let cancel = Arc::new(AtomicBool::new(false));
        let cancel_clone = cancel.clone();

        // Init WebSocket connection asynchronously because it's tungstenite
        spawn_local(async move {
            log::debug!("WebSocketService new(): init websocket thread spawned");
            let struct_sender = sender_clone;
            let struct_receiver = receiver_clone;
            let url = url_clone.clone();
            let wss = match connect(url).await {
                Ok(ws) => ws,
                Err(e) => {
                    cancel_clone.store(true, Ordering::SeqCst);
                    log::error!("WebSocketService: error connecting to WebSocket: {:?}", e.to_string());
                    return;
                }
            };
            let (sender, receiver) = wss.split();
            
            log::debug!("init ws thread: acquiring two locks...");
            *struct_sender.lock().await = Some(sender);
            *struct_receiver.lock().await = Some(receiver);
            log::debug!("init ws thread: locks released");
        });
        
        // Handle incoming messages
        let receiver_clone = receiver.clone();
        let cancel_clone = cancel.clone();
        spawn_local(async move {
            log::debug!("WebSocketService: listening for messages....");
            let receiver = receiver_clone;
            loop {
                if cancel_clone.load(Ordering::SeqCst) {
                    log::debug!("WebSocketService: cancellation signal received, breaking loop...");
                    break;
                }
                
                if receiver.lock().await.is_none() {
                    log::debug!("WebSocketService: receiver is None, skipping...");
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }

                log::debug!("WebSocketService: receiver is not None, processing messages...");
                let receiver_lock = receiver.clone();
                log::debug!("WebSocketService: receiver_lock created");
                if let Some(receiver) = receiver_lock.lock().await.as_mut() {
                log::debug!("WebSocketService: receiver_lock acquired");
                    while let Some(msg) = receiver.next().await {
                        let msg = msg.unwrap();
                        let msg = serde_json::from_str::<ChatMessage>(&msg.into_text().unwrap()).unwrap();
                        on_message.emit(msg);
                    }
                }
                log::debug!("WebSocketService: receiver_lock released, starting next iter...");
            }
        });

        log::debug!("WebSocketService new() finished");
        
        Self { sender, _receiver: receiver, user_id, username, cancel }
    }

    pub async fn send_message(&mut self, message: &String) -> Option<tokio_tungstenite_wasm::Error> {
        let msg = ChatMessage {
            user_id: self.user_id,
            username: self.username.clone(),
            content: message.clone(),
            timestamp: chrono::Utc::now(),
        };
        let mut sender = self.sender.lock().await;
        if let Some(sender) = &mut *sender {
            match sender.send(Message::Text(serde_json::to_string(&msg).unwrap())).await {
                Ok(_) => None,
                Err(e) => Some(e),
            }
        } else {
            Some(tokio_tungstenite_wasm::Error::ConnectionClosed)
        }

        // match self.sender.send(Message::Text(serde_json::to_string(&msg).unwrap())).await {
        //     Ok(_) => None,
        //     Err(e) => Some(e),
        // }
    }
}

impl Drop for WebSocketService {
    fn drop(&mut self) {
        log::debug!("WebSocketService: drop called, setting cancellation signal...");
        self.cancel.store(true, Ordering::SeqCst);
    }
}

// pub struct WebSocketService {
//     ws: WebSocket,
//     user_id: i32,
//     username: String,
// }

// impl WebSocketService {
//     pub fn new(url: &str, user_id: i32, username: String, on_message: Callback<ChatMessage>) -> Self {
//         let ws = WebSocket::new(url).unwrap();
        
//         // Handle incoming messages
//         let onmessage_callback = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
//             if let Some(txt) = e.data().as_string() {
//                 let msg = serde_json::from_str::<ChatMessage>(&txt).unwrap();
//                 on_message.emit(msg);
//             }
//         }) as Box<dyn FnMut(web_sys::MessageEvent)>);
//         ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
//         onmessage_callback.forget();
//         Self { ws, user_id, username }
//     }

//     pub fn send_message(&self, message: &String) -> Result<(), JsValue> {
//         todo!()
//         // self.ws.send_with_str(message)
//         // let msg = ChatMessage {
//         //     user_id: self.user_id,
//         //     username: self.username.clone(),
//         //     content: message.clone(),
//         //     addr: self.ws.
//         // };
//     }
// }

