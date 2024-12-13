// src/types/mod.rs
pub mod auth;
pub mod chat_room;

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub online: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChatRoom {
    pub id: String,
    pub name: String,
    pub users: Vec<User>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub user: User,
    pub timestamp: String,
}