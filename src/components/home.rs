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
    let chat_rooms = use_state(Vec::<ChatRoom>::new);
    let auth_ctx = use_context::<Rc<AuthContext>>().expect("Could not find AuthContext");
    let is_logged_in = use_state(|| { auth_ctx.state.is_authenticated }); // auth check
    let filter = use_state(String::new);
    let navigator = use_navigator().unwrap();

    let filtered_rooms = chat_rooms
        .iter()
        .filter(|room| {
            room.name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
        .collect::<Vec<_>>();

    let on_logout = move |_: MouseEvent| {
        let auth_ctx_clone = auth_ctx.clone();
        let navigator = navigator.clone();
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

            <div class="search-container">
                <input 
                    type="text"
                    placeholder="Search rooms..."
                    value={(*filter).clone()}
                    onchange={let filter = filter.clone(); move |e: Event| {
                        let target = e.target().unwrap();
                        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        filter.set(input.value());
                    }}
                />
            </div>

            <div class="rooms-grid">
                {if filtered_rooms.is_empty() {
                    html! {
                        <div class="no-rooms">
                            {"No chat rooms available"}
                        </div>
                    }
                } else {
                    filtered_rooms.iter().map(|room| {
                        html! {
                            <Link<Route> to={Route::ChatRoom { id: room.id.clone() }} classes="room-card">
                                <h3>{&room.name}</h3>
                                <p class="user-count">
                                    {format!("{} users online", room.users.len())}
                                </p>
                            </Link<Route>>
                        }
                    }).collect::<Html>()
                }}
            </div>
        </div>
    </>
    }
}
