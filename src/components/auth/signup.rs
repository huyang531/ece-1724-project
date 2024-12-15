use wasm_bindgen::JsCast;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yew::platform::spawn_local;
use crate::Route;
use crate::components::layout::Header;
use crate::services::auth;

#[function_component]
pub fn SignUp() -> Html {
    let username = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let confirm_password = use_state(|| String::new());
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let error = error.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let email = (*email).clone();
            let password = (*password).clone();
            let navigator = navigator.clone();
            let error = error.clone();
            let loading = loading.clone();

            // Basic validation
            if password.as_str() != confirm_password.as_str() {
                error.set(Some("Passwords do not match".to_string()));
                return;
            }

            if username.is_empty() || email.is_empty() || password.is_empty() {
                error.set(Some("All fields are required".to_string()));
                return;
            }

            loading.set(true);
            
            spawn_local(async move {
                match auth::signup(username, email, password).await {
                    Ok(_response) => {
                        log::info!("Signup successful");
                        window().unwrap().alert_with_message("Signup successful").unwrap();
                        // For now, just redirect to login
                        navigator.push(&Route::Login);
                    }
                    Err(err) => {
                        log::error!("Signup error: {:?}", err);
                        loading.set(false);
                        error.set(Some(err));
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
            <h2 class="auth-title">{"Create Account"}</h2>
            {if let Some(err) = (*error).clone() {
                html! { <div class="error">{err}</div> }
            } else {
                html! {}
            }}
            <form {onsubmit} class="auth-form">
                <div class="form-group">
                    <label for="username">{"Username"}</label>
                    <input 
                        type="text"
                        id="username"
                        value={(*username).clone()}
                        disabled={*loading}
                        onchange={let username = username.clone(); move |e: Event| {
                            let target = e.target().unwrap();
                            let input = target.dyn_into::<HtmlInputElement>().unwrap();
                            username.set(input.value());
                        }}
                    />
                </div>
                <div class="form-group">
                    <label for="email">{"Email"}</label>
                    <input 
                        type="email"
                        id="email"
                        value={(*email).clone()}
                        disabled={*loading}
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
                        type="password"
                        id="password"
                        value={(*password).clone()}
                        disabled={*loading}
                        onchange={let password = password.clone(); move |e: Event| {
                            let target = e.target().unwrap();
                            let input = target.dyn_into::<HtmlInputElement>().unwrap();
                            password.set(input.value());
                        }}
                    />
                </div>
                <div class="form-group">
                    <label for="confirm-password">{"Confirm Password"}</label>
                    <input 
                        type="password"
                        id="confirm-password"
                        value={(*confirm_password).clone()}
                        disabled={*loading}
                        onchange={let confirm_password = confirm_password.clone(); move |e: Event| {
                            let target = e.target().unwrap();
                            let input = target.dyn_into::<HtmlInputElement>().unwrap();
                            confirm_password.set(input.value());
                        }}
                    />
                </div>
                <button type="submit" class="auth-submit" disabled={*loading}>
                    {if *loading { "Creating Account..." } else { "Sign Up" }}
                </button>
            </form>
            <div class="auth-links">
                <Link<Route> to={Route::Login}>{"Already have an account? Login"}</Link<Route>>
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
