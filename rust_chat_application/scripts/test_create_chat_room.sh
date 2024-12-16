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

# Create a user first

curl -H "Content-Type: application/json" -d "{\"username\":\"World\",\"email\":\"lll@gmail.com\",\"password\":\"11111\"}" http://127.0.0.1:3000/api/user/signup
curl -H "Content-Type: application/json" -d "{\"username\":\"Hello\",\"email\":\"yyy@gmail.com\",\"password\":\"11111\"}" http://127.0.0.1:3000/api/user/signup
curl -H "Content-Type: application/json" -d "{\"username\":\"Hey..\",\"email\":\"zzz@gmail.com\",\"password\":\"11111\"}" http://127.0.0.1:3000/api/user/signup

# Create chat rooms with various names
create_chat_room 1 "ChatRoom1"
create_chat_room 1 "ChatRoom2"
create_chat_room 1 "ChatRoom3"
create_chat_room 1 ""          # Empty name
create_chat_room 1 "ChatRoom1" # Duplicate name
create_chat_room 2 "ChatRoom4"
create_chat_room 2 "ChatRoom5"
create_chat_room 2 ""          # Empty name
create_chat_room 2 "ChatRoom2" # Duplicate name
create_chat_room 3 "ChatRoom6"
create_chat_room 3 "ChatRoom7"
create_chat_room 3 ""          # Empty name
create_chat_room 3 "ChatRoom3" # Duplicate name
