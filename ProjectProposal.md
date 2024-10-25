# ECE1724-Rust Project Proposal

## Motivation
Designing the real-time chat application using Rust offers multiple motivations rooted in its performance, safety, and scalability. Rust’s high-performance capabilities, comparable to C and C++, allow for efficient handling of numerous concurrent WebSocket connections, a key requirement for real-time communication. Its memory safety model, which eliminates the need for a garbage collector, ensures predictable and smooth performance without pauses, a critical factor in applications where responsiveness is vital. Rust’s ownership and borrowing system also guarantees safe concurrency, preventing common issues like race conditions and data corruption, which are essential when managing multiple users and chat rooms. With asynchronous support via the async/await model, Rust allows the backend to scale effectively, handling thousands of concurrent users without blocking the system. The robust ecosystem of libraries like Tokio for async programming and WebSocket crates further simplifies building a high-performance chat application. Additionally, the option to use Yew for the front-end means the entire application can be developed consistently in Rust, making it a full-stack solution. These factors, combined with Rust’s strong reliability and safety guarantees, provide a compelling motivation to choose it for building a scalable, real-time chat application focused on raw performance.

## Objective and Key feature
Design and develop a high-performance, scalable real-time chat application that enables users to create rooms and send messages instantly. Key features include basic user authentication, chat room (or channel) creation and joining, and real-time messaging using WebSockets to ensure efficient data transfer. The application will also support presence detection, allowing users to see who is online or offline. A simple front-end user interface, which could be implemented as a command-line utility, will facilitate user interaction with the chat system. The focus will be on optimizing performance and ensuring the application scales effectively with increased usage.

## Tentative plan

### Work Distrubution Form
| Tasks      | Worker |
| ----------- | ----------- |
| User Authentication Module | Yingying Liu  |
| Instant Messaging  Module   | Yang Hu   |
| API and Database Design & Implementation  | Ze Yang  |

### User Interface User Authentication

TODO: Yingying Liu

#### User Authentication Module

For our Minimal Viable Product (MVP), the User Authentication Module should support the following features:

- User Login
- User Logout
- Fetch User Online Status
- User Sign Up
- Chat Room Management
  - Chat Room Creation
  - Chat Room Deleteion
  - Joining Chat Room
  - Leaving Chat Room

Tables:

- User
- ChatRoom
- UserInChatRoom
- Messages

### Instant Messaging Module

TODO: Yang Hu

https://www.zupzup.org/epoll-with-rust/index.html

The desgin of websocket will include two parts: client side and server side.

#### Basic requirements
* **Client Side:**

   Each client can connect with the server via the websocket. Also, the client can send the message and recieve the message during the session.
* **Server Side:** 

   The server mainly responsible for the communication between the clients. It will receive the messge from the client and broadcast it to other clients. The server will ensure that each client has an unique connection.
   

### API and Database Desgin

TODO: Ze Yang

