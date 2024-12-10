// src/services/websocket.rs
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;
use yew::Callback;

pub struct WebSocketService {
    ws: WebSocket,
}

impl WebSocketService {
    pub fn new(url: &str, on_message: Callback<String>) -> Result<Self, JsValue> {
        let ws = WebSocket::new(url)?;
        
        let on_message = on_message.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
            if let Some(txt) = e.data().as_string() {
                on_message.emit(txt);
            }
        }) as Box<dyn FnMut(web_sys::MessageEvent)>);
        
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        Ok(Self { ws })
    }

    pub fn send_message(&self, message: &str) -> Result<(), JsValue> {
        self.ws.send_with_str(message)
    }
}
