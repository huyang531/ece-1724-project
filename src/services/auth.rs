use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use crate::{config, types::auth::*};
use serde_wasm_bindgen::from_value;

pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    
    let login_request = LoginRequest { email, password };
    opts.body(Some(&JsValue::from_str(&serde_json::to_string(&login_request).unwrap())));

    let url = format!("{}{}", config::API_BASE_URL, config::Endpoints::LOGIN);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| err.as_string().unwrap())?;
    
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap())
        .await
        .map_err(|err| err.as_string().unwrap())?;
    let response: LoginResponse = from_value(json).unwrap();
    Ok(response)
}

pub async fn signup(username: String, email: String, password: String) -> Result<SignupResponse, String> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    
    let signup_request = SignupRequest { username, email, password };
    opts.body(Some(&JsValue::from_str(&serde_json::to_string(&signup_request).unwrap())));

    let url = format!("{}{}", config::API_BASE_URL, config::Endpoints::SIGNUP);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| err.as_string().unwrap())?;
    
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap())
        .await
        .map_err(|err| err.as_string().unwrap())?;
    
    let response: SignupResponse = from_value(json).unwrap();
    Ok(response)
}
