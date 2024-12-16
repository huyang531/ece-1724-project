use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};
use serde_wasm_bindgen::from_value;

use crate::{config, types::auth::*};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub user_id: i32,
    pub username: String,
}

pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    
    let login_request = LoginRequest { email, password };
    opts.set_body(Some(&JsValue::from_str(&serde_json::to_string(&login_request).expect("Failed to serialize login request: {}"))).expect("Failed to set body"));

    let url = format!("{}{}", config::API_BASE_URL, config::Endpoints::LOGIN);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Request failed. Is server started?".to_string()))?;

    let resp: Response = resp_value.dyn_into().unwrap();

    match resp.status() {
        401 => {
            return Err("Invalid email or password".to_string());
        }
        500 => {
            return Err("Internal server error".to_string());
        }
        _ => {}
    }

    let json = JsFuture::from(resp.json().unwrap())
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Error decoding json".to_string()))?;
    match from_value::<LoginResponse>(json.clone()) {
        Ok(response) => Ok(response),
        Err(err) => {
            log::error!("Failed to parse login response: {:?}", err);
            return Err(err.to_string());
        }
    }
}

pub async fn signup(username: String, email: String, password: String) -> Result<SignupResponse, String> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    
    let signup_request = SignupRequest { username, email, password };
    opts.set_body(Some(&JsValue::from_str(&serde_json::to_string(&signup_request).unwrap())).unwrap());

    let url = format!("{}{}", config::API_BASE_URL, config::Endpoints::SIGNUP);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Request failed. Is server started?".to_string()))?;
    
    let resp: Response = resp_value.dyn_into().unwrap();

    match resp.status() {
        400 => {
            return Err("User already exists".to_string());
        }
        500 => {
            return Err("Internal server error".to_string());
        }
        _ => {}
    }

    let json = JsFuture::from(resp.json().unwrap())
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Error decoding json".to_string()))?;
    
    match from_value::<SignupResponse>(json.clone()) {
        Ok(response) => Ok(response),
        Err(err) => {
            log::error!("Failed to parse signup response: {:?}", err);
            return Err(err.to_string());
        }
    }
}

pub async fn logout(user_id: i32) -> Result<(), String> {
    let logout_request = LogoutRequest { user_id };
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(Some(&JsValue::from_str(&serde_json::to_string(&logout_request).unwrap())).unwrap());

    let url = format!("{}{}", config::API_BASE_URL, config::Endpoints::LOGOUT);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Request failed. Is server started?".to_string()))?;

    let resp: Response = resp_value.dyn_into().unwrap();

    match resp.status() {
        401 => {
            return Err("Unauthorized".to_string());
        }
        500 => {
            return Err("Internal server error".to_string());
        }
        _ => {}
    }

    Ok(())
}

pub fn load_auth_token() -> Option<(i32, String)> {
    let window = window().unwrap();
    let storage = window.session_storage().unwrap().unwrap();
 
    match storage.get_item("user_token") {
        Ok(Some(token)) => {
            log::debug!("In load_auth_token(): Token: {:?}", token);
            let token: Token = serde_json::from_str(&token).expect("Failed to parse token");
            Some((token.user_id, token.username))
        }
        _ => {
            log::debug!("In load_auth_token(): No token found");
            None
        }
    }
}
