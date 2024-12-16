use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use futures::{lock::Mutex, pin_mut, stream::{SplitSink, SplitStream}, FutureExt, SinkExt, StreamExt};
use wasm_bindgen_futures::spawn_local;
use tokio_tungstenite_wasm::{connect, WebSocketStream, Message};
use yew::{platform::time::sleep, Callback};

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
            log::debug!("init ws thread: locks released. Sender and Receiver set.");
        });
        
        // Handle incoming messages
        let receiver_clone = receiver.clone();
        let cancel_clone = cancel.clone();
        spawn_local(async move {
            log::debug!("WebSocketService: listening for messages....");
            let receiver = receiver_clone;
            loop {
                if cancel_clone.load(Ordering::SeqCst) {
                    log::debug!("WebSocketService: cancellation signal received, killing WebSocketService...");
                    break;
                }
                        
                let mut receiver_lock = receiver.lock().await;
                if receiver_lock.is_none() {
                    drop(receiver_lock);
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }

                let cancel_fut = is_cancelled(cancel_clone.clone()).fuse();
                let mut receiver_fut = receiver_lock.as_mut().unwrap().next().fuse();

                pin_mut!(cancel_fut);

                futures::select! {
                    cancel = cancel_fut => {
                        if cancel {
                            log::debug!("WebSocketService: cancellation signal received, exiting...");
                            return;
                        }
                    },
                    msg = receiver_fut => {
                        if let Some(msg) = msg {
                            let msg = msg.unwrap();
                            let msg = msg.into_text().unwrap();
                            let msg = serde_json::from_str::<ChatMessage>(&msg.as_str()).unwrap_or_else(|_| {
                                log::error!("WebSocketService: failed to parse message: {:?}", msg);
                                ChatMessage::default()
                            });
                            on_message.emit(msg);
                        }
                    },
                };
            };
        });

        log::debug!("WebSocketService new() finished");
        
        Self { sender, _receiver: receiver, user_id, username, cancel }
    }

    pub async fn send_message(&mut self, message: &String) -> Option<tokio_tungstenite_wasm::Error> {
        log::debug!("WebSocketService: send_message() called");
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
            log::error!("WebSocketService: sender is None, returning ConnectionClosed error");
            Some(tokio_tungstenite_wasm::Error::ConnectionClosed)
        }
    }

    #[allow(dead_code)]
    pub fn close(&self) {
        log::debug!("WebSocketService: close() called, setting cancellation signal...");
        self.cancel.store(true, Ordering::SeqCst);
    }
}

// Cancel listener
async fn is_cancelled(cancel_clone: Arc<AtomicBool>) -> bool {
    loop {
        if cancel_clone.load(Ordering::SeqCst) {
            return true;
        }
        sleep(Duration::from_millis(100)).await;
    }
}

impl Drop for WebSocketService {
    fn drop(&mut self) {
        log::debug!("WebSocketService: drop called, setting cancellation signal...");
        self.cancel.store(true, Ordering::SeqCst);
    }
}
