use std::rc::Rc;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use web_sys::window;
use wasm_bindgen::JsCast;
use crate::Route;
use crate::services::auth::Token;
use crate::components::layout::Header;
use crate::services::auth;
use crate::context::auth::AuthContext;

#[function_component]
pub fn Login() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    
    let navigator = use_navigator().unwrap();
    let error = use_state(|| Option::<String>::None);
    let auth_ctx = use_context::<Rc<AuthContext>>().expect("Could not find AuthContext");

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        let storage = window().unwrap().session_storage().unwrap().unwrap();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = (*email).clone();
            let password = (*password).clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let auth_ctx = auth_ctx.clone();
            let storage = storage.clone();

            spawn_local(async move {
                match auth::login(email, password).await {
                    Ok(response) => {
                        let token = Token {
                            user_id: response.uid,
                            username: response.username.clone(),
                        };
                        auth_ctx.login.emit((response.uid, response.username));
                        log::info!("Login successful");
                        window().unwrap().alert_with_message("Login successful").unwrap();
                        let token = serde_json::to_string(&token).unwrap();
                        log::debug!("user_token: {}", token);
                        storage.set_item("user_token", &token).unwrap();
                        navigator.push(&Route::Home);
                    }
                    Err(err) => {
                        log::error!("Login error: {:?}", err);
                        // Show a popup that displays the error message
                        error.set(Some(err));
                        // window().unwrap().alert_with_message(&err).unwrap();
                    }
                }
            });
        })
    };

     // Error handler
     let on_close_error = {
        let error = error.clone();
        Callback::from(move |_| error.set(None))
    };

    html! {
        <>
            <Header />
            <div class="auth-container">
                <h2 class="auth-title">{"Welcome Back"}</h2>
                <form class="auth-form" {onsubmit}>
                    <div class="form-group">
                        <label for="email">{"Email"}</label>
                        <input 
                            id="email"
                            type="text"
                            placeholder="Enter your email"
                            value={(*email).clone()}
                            onchange={let email = email.clone(); move |e: Event| {
                                let target = e.target().unwrap();
                                let input = target.dyn_into::<HtmlInputElement>().unwrap();
                                email.set(input.value());
                            }}
                        />
                    </div>
                    <div class="form-group">
                        <label for="password">{"Password"}</label>
                        <input 
                            id="password"
                            type="password"
                            placeholder="Enter your password"
                            value={(*password).clone()}
                            onchange={let password = password.clone(); move |e: Event| {
                                let target = e.target().unwrap();
                                let input = target.dyn_into::<HtmlInputElement>().unwrap();
                                password.set(input.value());
                            }}
                        />
                    </div>
                    <button type="submit" class="auth-submit">{"Login"}</button>
                </form>
                <div class="auth-links">
                    <Link<Route> to={Route::SignUp}>{"Don't have an account? Sign up"}</Link<Route>>
                </div>
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
