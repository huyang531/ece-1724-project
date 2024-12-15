use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio_tungstenite_wasm::WebSocketStream;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::WebSocket;

use crate::config;
use crate::context::auth::AuthContext;
use crate::services::websocket::WebSocketService;
use crate::types::chat::ChatMessage;
use crate::components::layout::Header;
use crate::services::auth;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

pub struct ChatRoom
{
    messages: Vec<ChatMessage>,
    wss: Option<WebSocketService>,
    // link: ComponentLink<Self>,
    current_message: String,
    is_authenticated: bool,
    // auth_ctx: ContextHandle<AuthContext>,
}

pub enum Msg {
    SendMessage,
    UpdateMessage(String),
    ReceiveMessage(ChatMessage),
    AuthContextHandler(AuthContext),
}


impl Component for ChatRoom {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // let (auth_ctx, _) = ctx.link().context(ctx.link().callback(Msg::AuthContextHandler)).unwrap();
        log::debug!("ChatRoom create() called");
        let (auth_ctx, _) = ctx.link().context::<Rc<AuthContext>>(Callback::noop()).unwrap();

        let auth_token = auth::load_auth_token();
        log::debug!("Auth token: {:?}", auth_token);
        if let Some(token) = auth_token {
            log::debug!("Auth token found: {:?}", token);
            if !auth_ctx.state.is_authenticated {
                log::debug!("Auth token found and user is not authenticated");
                // Set the auth context with the loaded token
                auth_ctx.login.emit(token);
            }
        }

        log::debug!("ChatRoom create() auth_ctx: {:?}", auth_ctx);
        let is_authenticated = auth_ctx.state.is_authenticated;
        log::debug!("ChatRoom create() is_authenticated: {:?}", is_authenticated);
        
        if !is_authenticated {
            return Self {
                messages: Vec::new(),
                wss: None,
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
            wss: Some(wss),
            current_message: String::new(),
            is_authenticated,
        }
    }


    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SendMessage => {
                match self.wss.as_mut() {
                    Some(ref mut wss) => {
                        wss.send_message(&self.current_message);
                    }
                    None => {
                        log::error!("WebSocket connection not established");
                        return false;
                    }
                };
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
            Msg::AuthContextHandler(auth_ctx) => {
                self.is_authenticated = auth_ctx.state.is_authenticated;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Check if user is logged in yet
        if !self.is_authenticated {
            return html! {
                <>
                    <Header />
                    <div class="room-container">
                        <h3>{"Please log in and join the Chat Room via the \"Join Chat Room\" portal"}</h3>
                    </div>
                </>
            }
        }

        let onsubmit = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::SendMessage
        });

        let oninput = ctx.link().callback(|e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            Msg::UpdateMessage(input.value())
        });
        
        
        // let messages = use_state(Vec::<ChatMessage>::new);
        // let current_message = use_state(|| String::new());
        // let ws = use_state(|| Option::<WebSocket>::None);
    
        // // WebSocket connection effect
        // use_effect_with(
        //     (),
        //     move |_| {
        //         // TODO: Implement WebSocket connection
        //         || {}
        //     },
        // );
    
        // let onsubmit = {
        //     let current_message = current_message.clone();
        //     Callback::from(move |e: SubmitEvent| {
        //         e.prevent_default();
        //         // TODO: Send message via WebSocket
        //         current_message.set(String::new());
        //     })
        // };
    
        html! {
            <>
                <Header />
                <div class="chat-room-container">
                    <div class="chat-window">
                        <div class="messages">
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
                            // Messages will be displayed here
                        </div>
                    </div>
                    <div class="send-message-box">
                        <input
                            type="text"
                            placeholder="Type your message..."
                            class="message-input"
                            value={self.current_message.clone()}
                            oninput={oninput}
                        />
                        <button class="send-button" onclick={onsubmit}>{"Send"}</button>
                    </div>
                </div>
            </>
        }
    
            // html! {
            //     <div class="chat-room">
            //         <div class="messages">
            //             {messages.iter().map(|msg| html! {
            //                 <div class="message">
            //                     <span class="username">{&msg.user.username}</span>
            //                     <span class="content">{&msg.content}</span>
            //                 </div>
            //             }).collect::<Html>()}
            //         </div>
            //         <form {onsubmit}>
            //             <input 
            //                 type="text"
            //                 value={(*current_message).clone()}
            //                 onchange={let current_message = current_message.clone(); move |e: Event| {
            //                     let target = e.target().unwrap();
            //                     let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
            //                     current_message.set(input.value());
            //                 }}
            //             />
            //             <button type="submit">{"Send"}</button>
            //         </form>
            //     </div>
            // }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        log::debug!("ChatRoom destroy() called");
        // if let Some(wss) = self.wss.as_mut() {
        //     wss.close();
        // }
    } 
    
}