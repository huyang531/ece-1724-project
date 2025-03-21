#!/bin/bash

# Base URL for the API
BASE_URL="http://127.0.0.1:3000/api/chatrooms"

# Function to create a chat room
create_chat_room() {
    local user_id=$1
    local room_name=$2
    curl -H "Content-Type: application/json" -d "{\"user_id\":${user_id},\"room_name\":\"${room_name}\"}" ${BASE_URL}
    echo
}

# Create 4 users
curl -H "Content-Type: application/json" -d "{\"username\":\"Alice\",\"email\":\"alice@gmail.com\",\"password\":\"11111\"}" http://127.0.0.1:3000/api/user/signup
curl -H "Content-Type: application/json" -d "{\"username\":\"Bob\",\"email\":\"bob@gmail.com\",\"password\":\"11111\"}" http://127.0.0.1:3000/api/user/signup
curl -H "Content-Type: application/json" -d "{\"username\":\"Carol\",\"email\":\"carol@gmail.com\",\"password\":\"11111\"}" http://127.0.0.1:3000/api/user/signup
curl -H "Content-Type: application/json" -d "{\"username\":\"Yves\",\"email\":\"yves@gmail.com\",\"password\":\"11111\"}" http://127.0.0.1:3000/api/user/signup

# Create 3 Chat Rooms
create_chat_room 1 "ChatRoom1"
create_chat_room 2 "ChatRoom2"
create_chat_room 3 "ChatRoom3"
