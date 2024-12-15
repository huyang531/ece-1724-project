// src/config.rs
pub const API_BASE_URL: &str = "http://localhost:3000";
pub const WS_BASE_URL: &str = "ws://localhost:3000/ws/";

pub struct Endpoints;
impl Endpoints {
    pub const LOGIN: &'static str = "/api/user/login";
    pub const SIGNUP: &'static str = "/api/user/signup";
    pub const LOGOUT: &'static str = "/api/user/logout";
    pub const CREATE_CHAT_ROOM: &'static str = "/api/chatrooms";
    pub const JOIN_CHAT_ROOM: &'static str = "/api/chatrooms/join";
}