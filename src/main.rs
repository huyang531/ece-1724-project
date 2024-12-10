use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod services;
mod types;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/signup")]
    SignUp,
    #[at("/chat/:id")]
    ChatRoom { id: String },
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
                <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <components::Home /> },
        Route::Login => html! { <components::auth::Login /> },
        Route::SignUp => html! { <components::auth::SignUp /> },
        Route::ChatRoom { id } => html! { <components::chat::ChatRoom id={id} /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}