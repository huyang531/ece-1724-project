use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::auth::AuthProvider;

mod components;
mod services;
mod types;
mod config;
mod context;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/signup")]
    SignUp,
    #[at("/chat/:id")]
    ChatRoom { id: i32 },
}

#[function_component]
fn App() -> Html {
    html! {
        <AuthProvider>
            <BrowserRouter>
                    <Switch<Route> render={switch} />
            </BrowserRouter>
        </AuthProvider>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <components::Home /> },
        Route::Login => html! { <components::auth::Login /> },
        Route::SignUp => html! { <components::auth::SignUp /> },
        Route::ChatRoom { id } => html! { <components::chat::ChatRoom id={id.to_string()} /> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}