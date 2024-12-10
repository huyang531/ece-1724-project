use wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::types::{Message, User};
use web_sys::WebSocket;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component]
pub fn ChatRoom(props: &Props) -> Html {
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
        <div class="chat-room">
            <div class="messages">
                {messages.iter().map(|msg| html! {
                    <div class="message">
                        <span class="username">{&msg.user.username}</span>
                        <span class="content">{&msg.content}</span>
                    </div>
                }).collect::<Html>()}
            </div>
            <form {onsubmit}>
                <input 
                    type="text"
                    value={(*current_message).clone()}
                    onchange={let current_message = current_message.clone(); move |e: Event| {
                        let target = e.target().unwrap();
                        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        current_message.set(input.value());
                    }}
                />
                <button type="submit">{"Send"}</button>
            </form>
        </div>
    }
}