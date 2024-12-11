use yew::platform::spawn_local;
// src/components/auth/login.rs
use yew::prelude::*;
use yew_router::{navigator, prelude::*};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use crate::Route;
use crate::components::layout::Header;
use crate::services::auth;

#[function_component]
pub fn Login() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        // let error = error.clone();
        let navigator = navigator.clone();

        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // TODO: Implement login logic
            let email = (*email).clone();
            let password = (*password).clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                match auth::login(email, password).await {
                    Ok(response) => {
                        if response.status == "Success" {
                            // Store token
                            log::info!("Login successful");
                        } else {
                            log::error!("Login failed: {}", response.status);
                        }
                        navigator.push(&Route::Home);
                    }
                    Err(err) => {
                        log::error!("Login error: {:?}", err);
                    }
                }
            });
            // log::info!("Login attempt: {}", *email);
        })
    };

html! {
<>
    <Header />
    <div class="auth-container">
        // <button 
        //         onclick={let navigator = navigator.clone(); move |_| { navigator.back(); }}
        //         class="back-button"
        //     >
        //         {"← Back"}
        //     </button>
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
    </div>
</>
}
}
