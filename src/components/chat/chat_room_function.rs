use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::platform::spawn_local;
use web_sys::WebSocket;

use crate::context::auth::AuthContext;
use crate::services::websocket::WebSocketService;
use crate::types::chat::ChatMessage;
use crate::components::layout::Header;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(ChatRoom)]
pub fn chat_room(props: &Props) -> Html {
    let auth_ctx = use_context::<Rc<AuthContext>>().expect("Could not find AuthContext");

    let is_authenticated = auth_ctx.state.is_authenticated;

    let messages = use_state(|| Vec::new());
    let current_message = use_state(|| String::new());

    let on_message = {
        let messages = messages.clone();
        Callback::from(move |msg: ChatMessage| {
            messages.set({
                let mut msgs = (*messages).clone();
                msgs.push(msg);
                msgs
            });
        })
    };

    let websocket_service = use_state(|| {
        if is_authenticated {
            Some(WebSocketService::new(
                &format!("ws://localhost:8080/ws/{}", props.id),
                auth_ctx.state.user_id.unwrap(),
                auth_ctx.state.username.clone().unwrap(),
                Arc::new(on_message),
            ))
        } else {
            None
        }
    });

    let onsubmit = {
        let current_message = current_message.clone();
        let websocket_service = websocket_service.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(wss) = *websocket_service {
                let msg = (*current_message).clone();
                spawn_local(async move {
                    wss.await.send_message(&msg).await.unwrap();
                });
                current_message.set(String::new());
            }
        })
    };

    let oninput = {
        let current_message = current_message.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            current_message.set(input.value());
        })
    };

    if !is_authenticated {
        return html! {
            <>
                <Header />
                <div class="room-container">
                    <h3>{"Please log in and join the Chat Room via the \"Join Chat Room\" portal"}</h3>
                </div>
            </>
        };
    }

    html! {
        <>
            <Header />
            <div class="chat-room-container">
                <div class="chat-window">
                    <div class="messages">
                        { for (*messages).iter().map(|msg| html! {
                            <div class="message">
                                <span class="username">{ &msg.username }</span>
                                <span class="content">{ &msg.content }</span>
                            </div>
                        }) }
                    </div>
                </div>
                <form class="send-message-box" onsubmit={onsubmit}>
                    <input
                        type="text"
                        placeholder="Type your message..."
                        class="message-input"
                        value={(*current_message).clone()}
                        oninput={oninput}
                    />
                    <button type="submit" class="send-button">{"Send"}</button>
                </form>
            </div>
        </>
    }
}