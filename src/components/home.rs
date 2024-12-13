use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{window, Storage};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::{navigator, prelude::*};
use crate::types::ChatRoom;
use crate::services::chat_room;
use crate::Route;
use crate::components::layout::Header;
use crate::context::auth::AuthContext;
use crate::services::auth;

fn load_auth_token() -> Option<String> {
    let window = window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
 
    storage.get_item("user_id").unwrap()
}

#[function_component]
pub fn Home() -> Html {
    // Fetch login status from session storage
    let auth_ctx = use_context::<Rc<AuthContext>>().expect("Could not find AuthContext");
    let auth_token = load_auth_token();
    if let Some(token) = auth_token {
        if !auth_ctx.state.is_authenticated {
            // Set the auth context with the loaded token
            auth_ctx.login.emit(token.parse().unwrap());
        }
    }

    let auth_ctx_clone = auth_ctx.clone();

    let is_logged_in = use_state(|| { auth_ctx.state.is_authenticated }); // auth check
    {
        let is_logged_in = is_logged_in.clone();
        let auth_ctx = auth_ctx.clone();
        use_effect_with(auth_ctx.state.is_authenticated, move |value| {
            is_logged_in.set(value.clone());
        });
    }
    let _user_id = use_state(|| { auth_ctx.state.user_id.clone() });

    let navigator = use_navigator().unwrap();
    let navigator_clone = navigator.clone();

    let error = use_state(|| Option::<String>::None);
    let error_clone = error.clone();

    let room_id = use_state(|| 0);
    let room_name = use_state(String::new);
    let room_name_clone = room_name.clone();
        
    // Error handler
    let on_close_error = {
        let error = error_clone.clone();
        Callback::from(move |_| error.set(None))
    };
    let error_clone = error.clone();
    
    
    
    let on_create_chat_room = move |_: MouseEvent| {
        let auth_ctx = auth_ctx_clone.clone();
        let navigator = navigator_clone.clone();
        let room_name = room_name_clone.clone();
        let error = error_clone.clone();

        spawn_local(async move {
            // let auth_ctx = auth_ctx_clone.clone();
            let navigator = navigator.clone();
            let room_name = room_name.clone();
            match chat_room::create_chat_room(auth_ctx.state.user_id.clone().expect("User is not logged in!"), (*room_name).clone()).await {
                Ok(response) => {
                    log::info!("Chat room created: {:?}", response.room_id);
                    window().unwrap().alert_with_message(format!("Chat room created successfully. Room ID: {:?}", response.room_id).as_str()).unwrap();
                    // navigator.push(&Route::ChatRoom { id: response.room_id });
                }
                Err(err) => {
                    log::error!("Failed to create chat room: {:?}", err);
                    error.set(Some(err));
                    // window().unwrap().alert_with_message(&err).unwrap();
                }
            }
        });
    };
    let error_clone = error.clone();
    let navigator_clone = navigator.clone();
    let auth_ctx_clone = auth_ctx.clone();


    let on_logout = move |_: MouseEvent| {
        let auth_ctx_clone = auth_ctx_clone.clone();
        let navigator = navigator_clone.clone();
        let error = error_clone.clone();
        let storage = window().unwrap().local_storage().unwrap().unwrap();
        
        spawn_local(async move {
            let auth_ctx = auth_ctx_clone.clone();
            let navigator = navigator.clone();
            match auth::logout(auth_ctx.state.user_id.clone().expect("User is not logged in!")).await {
                Ok(_) => {
                    log::info!("Logout successful");
                    auth_ctx_clone.logout.emit(());
                    window().unwrap().alert_with_message("Logout successful").unwrap();
                    storage.remove_item("user_id").unwrap();
                    navigator.push(&Route::Login);
                }
                Err(err) => {
                    log::error!("Logout error: {:?}", err);
                    error.set(Some(err));
                    // window().unwrap().alert_with_message(&err).unwrap();
                }
            }
        });
    };
    let error_clone = error.clone();

    html! {
    <>
        <Header />
        <div class="home-container">
            <header class="home-header">
                <h1>{"Welcome to RustChat"}</h1>
                {if *is_logged_in {
                    html! {
                        <div class="auth-buttons">
                            <button class="btn-secondary" onclick={on_logout}>
                                {"Logout"}
                            </button>
                        </div>
                    }
                } else {
                    html! {
                        <div class="auth-buttons">
                            <Link<Route> to={Route::Login} classes="btn-primary">
                                {"Login"}
                            </Link<Route>>
                            <Link<Route> to={Route::SignUp} classes="btn-secondary">
                                {"Sign Up"}
                            </Link<Route>>
                        </div>
                    }
                }}
            </header>
            {if *is_logged_in {
                html! {
                <div class="chat-row">
                    <div class="chat-container">
                        <h2 class="chat-title">{"Create a Chat Room"}</h2>
                        <form class="chat-form">
                            <div class="form-group">
                                <label for="chatroom_name">{"Chat Room Name"}</label>
                                <input 
                                    type="text"
                                    placeholder="Enter Chat Room Name..."
                                    onchange={let room_name = room_name.clone(); move |e: Event| {
                                        let target = e.target().unwrap();
                                        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                        room_name.set(input.value());
                                    }}
                                />
                            </div>
                            <button class="chat-submit" onclick={on_create_chat_room}>
                                {"Create"}
                            </button>
                        </form>
                    </div>

                    <div class="chat-container">
                        <h2 class="chat-title">{"Join a Chat Room"}</h2>
                        <form class="chat-form">
                            <div class="form-group">
                                <label for="chatroom_id">{"Chat Room ID"}</label>
                                <input 
                                    type="text"
                                    placeholder="Enter Chat Room ID..."
                                    // value={(*room_id).clone()}
                                    onchange={let room_id = room_id.clone(); let error = error_clone.clone(); move |e: Event| {
                                        let target = e.target().unwrap();
                                        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                        let id = match input.value().parse::<i32>() {
                                            Ok(id) => id,
                                            Err(_) => {
                                                error.set(Some("Invalid room ID".to_string()));
                                                return;
                                            }
                                        };
                                        room_id.set(id);
                                    }}
                                />
                            </div>
                            <button class="chat-submit" onclick={let navigator = navigator.clone(); let room_id = room_id.clone(); move |_| {
                                let id = (*room_id).clone();
                                // Navigate to the chatroom with the given ID
                                // TODO: Call join_chat_room() API?
                                navigator.push(&Route::ChatRoom { id });
                            }}>
                                {"Join"}
                            </button>
                        </form>
                    </div>
                </div>
                }
            } else {
                html! {
                    <div class="no-rooms">
                        {"Please log in to join a chat room!"}
                    </div>
                }
            }}
            if let Some(error_message) = (*error).clone() {
                        <div class="error-popup">
                            <div class="error-content">
                                <p>{error_message}</p>
                                <button onclick={on_close_error}>{"Close"}</button>
                            </div>
                        </div>
                }
        </div>
    </>
    }
}
