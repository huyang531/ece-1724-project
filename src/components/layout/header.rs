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
                let user_id = match &auth_ctx.state.user_id {
                    Some(user_id) => user_id.to_string(),
                    None => "Unknown".to_string(),
                };

                let username = match &auth_ctx.state.username {
                    Some(username) => username,
                    None => "Unknown",
                };

                format!("Welcome, {}! (ID: {})", username, user_id)
            }}</h4>
        </header>
    }
}
