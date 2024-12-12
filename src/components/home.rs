use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::window;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::{navigator, prelude::*};
use crate::types::ChatRoom;
use crate::Route;
use crate::components::layout::Header;
use crate::context::auth::AuthContext;
use crate::services::auth;

#[function_component]
pub fn Home() -> Html {
    let auth_ctx = use_context::<Rc<AuthContext>>().expect("Could not find AuthContext");
    let is_logged_in = use_state(|| { auth_ctx.state.is_authenticated }); // auth check
    let navigator = use_navigator().unwrap();
    let navigator_clone = navigator.clone();
        
    // let chat_rooms = use_state(Vec::<ChatRoom>::new);
    // let filter = use_state(String::new);
    // let filtered_rooms = chat_rooms
    // .iter()
    // .filter(|room| {
    //     room.name
    //     .to_lowercase()
    //     .contains(&filter.to_lowercase())
    // })
    // .collect::<Vec<_>>();

    let room_id = use_state(String::new);

    let on_logout = move |_: MouseEvent| {
        let auth_ctx_clone = auth_ctx.clone();
        let navigator = navigator_clone.clone();
        spawn_local(async move {
            let auth_ctx = auth_ctx_clone.clone();
            let navigator = navigator.clone();
            match auth::logout(auth_ctx.state.user_id.clone().unwrap_or_else(|| "User is not logged in!".to_string())).await {
                Ok(_) => {
                    log::info!("Logout successful");
                    auth_ctx_clone.logout.emit(());
                    window().unwrap().alert_with_message("Logout successful").unwrap();
                    navigator.push(&Route::Login);
                }
                Err(err) => {
                    log::error!("Logout error: {:?}", err);
                    window().unwrap().alert_with_message(&err).unwrap();
                }
            }
        });
    };

    html! {
    <>
        <Header />
        <div class="home-container">
            <header class="home-header">
                <h1>{"Welcome to RustChat"}</h1>
                {if *is_logged_in {
                    html! {
                        <div class="user-controls">
                            <button class="btn-primary">{"Create New Room"}</button>
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
                <div class="auth-container">
                    <h2 class="auth-title">{"Join a Chat Room"}</h2>
                    <form class="auth-form">
                        <div class="form-group">
                            <label for="chatroom_id">{"Chat Room ID"}</label>
                            <input 
                                type="text"
                                placeholder="Enter Chat Room ID..."
                                value={(*room_id).clone()}
                                onchange={let room_id = room_id.clone(); move |e: Event| {
                                    let target = e.target().unwrap();
                                    let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                    room_id.set(input.value());
                                }}
                            />
                            <button class="auth-submit" onclick={let navigator = navigator.clone(); let room_id = room_id.clone(); move |_| {
                                let id = (*room_id).clone();
                                // Navigate to the chatroom with the given ID
                                navigator.push(&Route::ChatRoom { id });
                            }}>
                                {"Join"}
                            </button>
                        </div>
                    </form>
                </div>
                }
            } else {
                html! {
                    <div class="no-rooms">
                        {"Please log in to join a chat room!"}
                    </div>
                }
            }}
        </div>
    </>
    }
}
