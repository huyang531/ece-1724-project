use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component]
pub fn Header() -> Html {
    let auth_ctx = use_context::<Rc<crate::context::auth::AuthContext>>().expect("Could not find AuthContext");
    html! {
        <header class="app-header">
            <Link<Route> to={Route::Home} classes="logo">
                {"RustChat"}
            </Link<Route>>
            <h4>{{
                if let Some(user_id) = &auth_ctx.state.user_id {
                    format!("Welcome, {}! (ID: {})", auth_ctx.state.username.as_ref().unwrap_or(&"Unknown".to_string()), user_id)
                } else {
                    "Welcome, please log in!".to_string()
                }
            }}</h4>
        </header>
    }
}
