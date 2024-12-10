// src/components/auth/login.rs
use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[function_component]
pub fn Login() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // TODO: Implement login logic
            log::info!("Login attempt: {}", *username);
        })
    };

    html! {
        <div class="login-container">
            <form {onsubmit}>
                <input 
                    type="text"
                    placeholder="Username"
                    value={(*username).clone()}
                    onchange={let username = username.clone(); move |e: Event| {
                        let target = e.target().unwrap();
                        let input = target.dyn_into::<HtmlInputElement>().unwrap();
                        username.set(input.value());
                    }}
                />
                <input 
                    type="password"
                    placeholder="Password"
                    value={(*password).clone()}
                    onchange={let password = password.clone(); move |e: Event| {
                        let target = e.target().unwrap();
                        let input = target.dyn_into::<HtmlInputElement>().unwrap();
                        password.set(input.value());
                    }}
                />
                <button type="submit">{"Login"}</button>
            </form>
        </div>
    }
}
