use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use crate::{config, types::chat_room::*};
use serde_wasm_bindgen::from_value;

pub async fn create_chat_room(user_id: i32, room_name: String) -> Result<CreateChatRoomResponse, String> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);

    let create_request = CreateChatRoomRequest { user_id, room_name };
    opts.set_body(Some(&JsValue::from_str(&serde_json::to_string(&create_request).unwrap())).unwrap());

    let url = format!("{}{}", config::API_BASE_URL, config::Endpoints::CREATE_CHAT_ROOM);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Request failed. Is server started?".to_string()))?;
    let resp = resp_value.dyn_into::<Response>().unwrap();

    match resp.status() {
        401 => {
            return Err("Unauthorized".to_string());
        }
        500 => {
            return Err("Internal server error. Please try logging in again!".to_string());
        }
        _ => {}
    }

    let json = JsFuture::from(resp.json().unwrap())
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Error decoding json".to_string()))?;
    match from_value::<CreateChatRoomResponse>(json.clone()) {
        Ok(response) => Ok(response),
        Err(err) => {
            log::error!("Failed to parse create chat room response: {:?}", err);
            return Err(err.to_string());
        }
    }
}

pub async fn join_chat_room(user_id: i32, room_id: i32) -> Result<JoinChatRoomResponse, String> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    
    let join_request = JoinChatRoomRequest { user_id, room_id };
    opts.set_body(Some(&JsValue::from_str(&serde_json::to_string(&join_request).unwrap())).unwrap());

    let url = format!("{}{}", config::API_BASE_URL, config::Endpoints::JOIN_CHAT_ROOM);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Content-Type", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Request failed. Is server started?".to_string()))?;

    let resp: Response = resp_value.dyn_into().unwrap();

    match resp.status() {
        404 => {
            return Err("Room not found".to_string());
        }
        401 => {
            return Err("Unauthorized".to_string());
        }
        500 => {
            return Err("Internal server error. Please try logging in again!".to_string());
        }
        _ => {}
    }

    let json = JsFuture::from(resp.json().unwrap())
        .await
        .map_err(|err| err.as_string().unwrap_or_else(|| "Error decoding json".to_string()))?;
    match from_value::<JoinChatRoomResponse>(json.clone()) {
        Ok(response) => Ok(response),
        Err(err) => {
            log::error!("Failed to parse join chat room response: {:?}", err);
            return Err(err.to_string());
        }
    }
}
