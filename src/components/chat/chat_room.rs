use std::rc::Rc;

use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::WebSocket;

use crate::context::auth::AuthContext;
use crate::types::Message;
use crate::components::layout::Header;
use crate::services::auth;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component]
pub fn ChatRoom(props: &Props) -> Html {
    let auth_ctx = use_context::<Rc<AuthContext>>().expect("Could not find AuthContext");

    // Fetch login status from session storage
    // let auth_token = auth::load_auth_token();
    // if let Some(token) = auth_token {
    //     if !auth_ctx.state.is_authenticated {
    //         // Set the auth context with the loaded token
    //         auth_ctx.login.emit(token.parse().unwrap());
    //     }
    // }
    
    // Check if user is logged in yet
    if auth_ctx.state.user_id.is_none() {
        return html! {
            <>
                <Header />
                <div class="room-container">
                    <h3>{"Please log in and join the Chat Room via the \"Join Chat Room\" portal"}</h3>
                </div>
            </>
        }
    }

    // Establish WebSocket connection
    let ws = use_state(|| {
        let ws = WebSocket::new("ws://localhost:8080/ws").unwrap();
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        ws
    });

    let messages = use_state(Vec::<Message>::new);
    let current_message = use_state(|| String::new());
    let ws = use_state(|| Option::<WebSocket>::None);

    // WebSocket connection effect
    use_effect_with(
        (),
        move |_| {
            // TODO: Implement WebSocket connection
            || {}
        },
    );

    let onsubmit = {
        let current_message = current_message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // TODO: Send message via WebSocket
            current_message.set(String::new());
        })
    };

    html! {
        <>
            <Header />
            <div class="chat-room-container">
                <div class="chat-window">
                    <div class="messages">
                        // Messages will be displayed here
                    </div>
                </div>
                <div class="send-message-box">
                    <input type="text" placeholder="Type your message..." class="message-input" />
                    <button class="send-button">{"Send"}</button>
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