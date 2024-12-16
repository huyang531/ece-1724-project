use std::rc::Rc;
use std::sync::Arc;

use futures::lock::Mutex;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::RouterScopeExt;

use crate::{config, Route};
use crate::context::auth::AuthContext;
use crate::services::websocket::WebSocketService;
use crate::types::chat::ChatMessage;
use crate::components::layout::Header;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

pub struct ChatRoom
{
    messages: Vec<ChatMessage>,
    wss: Arc<Mutex<Option<WebSocketService>>>,
    current_message: String,
    is_authenticated: bool,
}

pub enum Msg {
    SendMessage,
    UpdateMessage(String),
    ReceiveMessage(ChatMessage),
    LeaveRoom,
}


impl Component for ChatRoom {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        log::debug!("ChatRoom create() called");
        let (auth_ctx, _) = ctx.link().context::<Rc<AuthContext>>(Callback::noop()).unwrap();

        let is_authenticated = auth_ctx.state.is_authenticated;
        
        if !is_authenticated {
            return Self {
                messages: Vec::new(),
                wss: Arc::new(Mutex::new(None)),
                current_message: String::new(),
                is_authenticated: false,
            }
        }

        let link = ctx.link().clone();
        let on_message = link.callback(Msg::ReceiveMessage);

        let wss = WebSocketService::new(
            &format!("{}{}?user_id={}&username={}", config::WS_BASE_URL, ctx.props().id, auth_ctx.state.user_id.unwrap(), auth_ctx.state.username.clone().unwrap()),
            auth_ctx.state.user_id.unwrap(),
            auth_ctx.state.username.clone().unwrap(),
            on_message,
        );

        log::debug!("ChatRoom create() finished");
        Self {
            messages: Vec::new(),
            wss: Arc::new(Mutex::new(Some(wss))),
            current_message: String::new(),
            is_authenticated,
        }
    }


    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let wss_clone = self.wss.clone();
        let message_clone = self.current_message.clone();
        match msg {
            Msg::SendMessage => {
                log::debug!("Msg::SendMessage received");
                spawn_local(async move {
                    let mut wss = wss_clone.lock().await;
                    match wss.as_mut() {
                        Some(ref mut wss) => {
                            if message_clone.is_empty() {
                                log::debug!("Message is empty, skipping...");
                                return;
                            }
                            log::debug!("Calling wss.send_message()...");
                            match wss.send_message(&message_clone).await {
                                Some(err) => {
                                    log::error!("Error sending message: {:?}", err);
                                }
                                None => {
                                    log::debug!("Message sent successfully");
                                }
                            }
                        }
                        None => {
                            log::error!("WebSocket connection not established");
                        }
                    }
                });
                self.current_message.clear();
                true
            }
            Msg::UpdateMessage(msg) => {
                self.current_message = msg;
                true
            }
            Msg::ReceiveMessage(msg) => {
                self.messages.push(msg);
                true
            }
            Msg::LeaveRoom => {
                log::debug!("Msg::LeaveRoom received");
                ctx.link().navigator().unwrap().push(&Route::Home);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Check if user is logged in yet
        if !self.is_authenticated {
            // ctx.link().navigator().unwrap().push(&Route::Home);
            return html! {
                <>
                    <Header />
                    <div class="room-container">
                        <h3>{"Please log in and join the Chat Room via the \"Join Chat Room\" portal"}</h3>
                    </div>
                </>
            };
        }

        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            log::debug!("Send message button clicked");
            Msg::SendMessage
        });

        let on_input = ctx.link().callback(|e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            log::debug!("Input event: {:?}", input.value());
            Msg::UpdateMessage(input.value())
        });

        let on_back = ctx.link().callback(|_: MouseEvent| {
            log::debug!("Back button clicked");
            Msg::LeaveRoom
        });
        
    
        html! {
            <>
                <Header />
                <div class="chat-room-container">
                    <div class="room-info">
                        <h2>{ format!("Room ID: {}", ctx.props().id) }</h2>
                        <button class="back-button" onclick={on_back}>{"Leave"}</button>
                    </div>
                    <div class="chat-window">
                        <div class="messages">
                        // Messages will be displayed here
                        {
                            for self.messages.iter().map(|msg| {
                                html! {
                                    <div class="message">
                                        <span class="username">{ &msg.username }</span>
                                        <span class="timestamp">{ format!("{} UTC", msg.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()) }</span>
                                        <span class="content">{ &msg.content }</span>
                                    </div>
                                }
                            })
                        }
                        </div>
                    </div>
                    <form class="send-message-box" onsubmit={on_submit}>
                        <input
                            type="text"
                            placeholder="Type your message..."
                            class="message-input"
                            value={self.current_message.clone()}
                            oninput={on_input}
                        />
                        <button type="submit" class="send-button">{"Send"}</button>
                    </form>
                </div>
            </>
        }
    }
}
