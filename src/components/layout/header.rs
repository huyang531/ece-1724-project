use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component]
pub fn Header() -> Html {
    // let theme = use_state(|| "light".to_string());
    
    // let toggle_theme = {
    //     let theme = theme.clone();
    //     Callback::from(move |_| {
    //         let new_theme = if *theme == "light" { "dark" } else { "light" };
    //         theme.set(new_theme.to_string());
    //         // Add theme to body class
    //         let window = web_sys::window().unwrap();
    //         let document = window.document().unwrap();
    //         let body = document.body().unwrap();
    //         body.set_attribute("theme", new_theme);
    //     })
    // };

    let auth_ctx = use_context::<Rc<crate::context::auth::AuthContext>>().expect("Could not find AuthContext");
    html! {
        <header class="app-header">
            <Link<Route> to={Route::Home} classes="logo">
                {"RustChat"}
            </Link<Route>>
            <h4>{
                match auth_ctx.state.user_id {
                    Some(user_id) => html! { format!("User ID: {:?}", user_id) },
                    None => html! {}
                }}</h4>
        </header>
    }
}