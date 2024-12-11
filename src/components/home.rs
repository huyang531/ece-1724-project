use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::types::ChatRoom;
use crate::Route;
use crate::components::layout::Header;

#[function_component]
pub fn Home() -> Html {
    let chat_rooms = use_state(Vec::<ChatRoom>::new);
    let is_logged_in = use_state(|| false); // TODO: Implement actual auth check
    let filter = use_state(String::new);

    let filtered_rooms = chat_rooms
        .iter()
        .filter(|room| {
            room.name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
        .collect::<Vec<_>>();

    html! {
    <>
        <Header />
        <div class="home-container">
            <header class="home-header">
                <h1>{"Welcome to RustChat"}</h1>
                {if *is_logged_in {
                    html! {
                        <div class="user-controls">
                            <button class="create-room-btn">{"Create New Room"}</button>
                            <Link<Route> to={Route::Login} classes="logout-btn">
                                {"Logout"}
                            </Link<Route>>
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
