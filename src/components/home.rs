use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::window;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::services::chat_room;
use crate::Route;
use crate::components::layout::Header;
use crate::context::auth::AuthContext;
use crate::services::auth;



#[function_component]
pub fn Home() -> Html {
    // Fetch login status from session storage
    let auth_ctx = use_context::<Rc<AuthContext>>().expect("Could not find AuthContext");
    let auth_token = auth::load_auth_token();
    log::debug!("Auth token: {:?}", auth_token);
    if let Some(token) = auth_token {
        log::debug!("Auth token found");
        if !auth_ctx.state.is_authenticated {
            log::debug!("Auth token found and user is not authenticated");
            // Set the auth context with the loaded token
            auth_ctx.login.emit(token);
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

    // // Improvement: Call logout API when the session terminates
    // {
    //     let auth_ctx = auth_ctx.clone();
    //     use_effect(move || {
    //         let window = window().unwrap();
    //         let storage = window.session_storage().unwrap().unwrap();
    //         let user_id = auth_ctx.state.user_id.clone();

    //         // Cleanup function to call logout API
    //         move || {
    //             if let Some(user_id) = user_id {
    //                 spawn_local(async move {
    //                     match auth::logout(user_id).await {
    //                         Ok(_) => {
    //                             log::info!("Logout successful");
    //                             storage.remove_item("user_id").unwrap();
    //                         }
    //                         Err(err) => {
    //                             log::error!("Logout error: {:?}", err);
    //                         }
    //                     }
    //                 });
    //             }
    //         }
    //     });
    // }

    let navigator = use_navigator().unwrap();
    let navigator_clone = navigator.clone();

    let error = use_state(|| Option::<String>::None);
    let error_clone = error.clone();

    let room_id = use_state(|| 0);
    let room_id_clone = room_id.clone();
    let room_name = use_state(String::new);
    let room_name_clone = room_name.clone();
        
    // Error handler
    let on_close_error = {
        let error = error_clone.clone();
        Callback::from(move |_| error.set(None))
    };
    let error_clone = error.clone();
    
    let on_join_chat_room = move |_: MouseEvent| {
        let navigator = navigator_clone.clone();
        let room_id = room_id_clone.clone();
        let user_id = auth_ctx_clone.state.user_id.clone().expect("User is not logged in!");
        let error = error_clone.clone();
        spawn_local(async move {
            let room_id = (*room_id).clone();
            let navigator = navigator.clone();
            let error = error.clone();
            match chat_room::join_chat_room(user_id, room_id).await {
                Ok(_) => {
                    log::info!("Joined chat room");
                    navigator.push(&Route::ChatRoom { id: room_id });
                }
                Err(err) => {
                    log::error!("Failed to join chat room: {:?}", err);
                    error.set(Some(err));
                }
            }
        });
    };
    let error_clone = error.clone();
    let navigator_clone = navigator.clone();
    let auth_ctx_clone = auth_ctx.clone();
    
    let on_create_chat_room = move |_: MouseEvent| {
        let auth_ctx = auth_ctx_clone.clone();
        let navigator = navigator_clone.clone();
        let room_name = room_name_clone.clone();
        let error = error_clone.clone();

        if room_name.is_empty() {
            error.set(Some("Room name cannot be empty".to_string()));
            return;
        }

        spawn_local(async move {
            // let auth_ctx = auth_ctx_clone.clone();
            let navigator = navigator.clone();
            let room_name = room_name.clone();
            match chat_room::create_chat_room(auth_ctx.state.user_id.clone().expect("User is not logged in!"), (*room_name).clone()).await {
                Ok(response) => {
                    log::info!("Chat room created: {:?}", response.room_id);
                    window().unwrap().alert_with_message(format!("Chat room created successfully. Room ID: {:?}", response.room_id).as_str()).unwrap();
                    navigator.push(&Route::ChatRoom { id: response.room_id });
                }
                Err(err) => {
                    log::error!("Failed to create chat room: {:?}", err);
                    error.set(Some(err));
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
        let storage = window().unwrap().session_storage().unwrap().unwrap();
        
        spawn_local(async move {
            let auth_ctx = auth_ctx_clone.clone();
            let navigator = navigator.clone();
            match auth::logout(auth_ctx.state.user_id.clone().expect("User is not logged in!")).await {
                Ok(_) => {
                    log::info!("Logout successful");
                    auth_ctx_clone.logout.emit(());
                    window().unwrap().alert_with_message("Logout successful").unwrap();
                    storage.remove_item("user_token").unwrap();
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
                            <button class="chat-submit" onclick={on_create_chat_room}>
                                {"Create"}
                            </button>
                        </div>
                    </div>

                    <div class="chat-container">
                        <h2 class="chat-title">{"Join a Chat Room"}</h2>
                        <div class="form-group">
                            <label>{"Chat Room ID"}</label>
                            <input 
                                type="number"
                                min="1"
                                placeholder="Enter Chat Room ID..."
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
                            <button class="chat-submit" onclick={on_join_chat_room}>
                                {"Join"}
                            </button>
                        </div>
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
