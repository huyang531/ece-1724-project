use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

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
            
            // TODO: Implement actual signup logic here
            // For now, just redirect to login
            navigator.push(&Route::Login);
        })
    };

    html! {
        <div class="signup-container">
            <h2>{"Create Account"}</h2>
            {if let Some(err) = (*error).clone() {
                html! { <div class="error">{err}</div> }
            } else {
                html! {}
            }}
            <form {onsubmit} class="signup-form">
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
                <button type="submit" disabled={*loading}>
                    {if *loading { "Creating Account..." } else { "Sign Up" }}
                </button>
            </form>
            <div class="auth-links">
                <Link<Route> to={Route::Login}>{"Already have an account? Login"}</Link<Route>>
            </div>
        </div>
    }
}
