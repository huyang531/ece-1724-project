# ECE1724-Rust Project Proposal

## Motivation
Designing the real-time chat application using Rust offers multiple motivations rooted in its performance, safety, and scalability. Rust's high-performance capabilities, comparable to C and C++, allow for efficient handling of numerous concurrent WebSocket connections, a key requirement for real-time communication. Its memory safety model, which eliminates the need for a garbage collector, ensures predictable and smooth performance without pauses, a critical factor in applications where responsiveness is vital. Rust's ownership and borrowing system also guarantees safe concurrency, preventing common issues like race conditions and data corruption, which are essential when managing multiple users and chat rooms. With asynchronous support via the async/await model, Rust allows the backend to scale effectively, handling thousands of concurrent users without blocking the system. The robust ecosystem of libraries like Tokio for async programming and WebSocket crates further simplifies building a high-performance chat application. Additionally, the option to use Yew for the front-end means the entire application can be developed consistently in Rust, making it a full-stack solution. These factors, combined with Rust's strong reliability and safety guarantees, provide a compelling motivation to choose it for building a scalable, real-time chat application focused on raw performance.

## Objective and Key feature
Design and develop a high-performance, scalable real-time chat application that enables users to create rooms and send messages instantly. Key features include basic user authentication, chat room (or channel) creation and joining, and real-time messaging using WebSockets to ensure efficient data transfer. The application will also support presence detection, allowing users to see who is online or offline. A simple front-end user interface, which could be implemented as a command-line utility, will facilitate user interaction with the chat system. The focus will be on optimizing performance and ensuring the application scales effectively with increased usage.

## Tentative plan

### Work Distrubution 

| Tasks      | Worker |
| ----------- | ----------- |
| System Design  | Ze Yang  |
| User Authentication Module | Yingying Liu  |
| Instant Messaging  Module   | Yang Hu   |

### System Design

TODO: Ze Yang
> 可以考虑加张图？

#### API Design

- Tools: Apifox
- Style: RESTful API
- Backend framework: 

#### Data Model

Tables:

- User
- ChatRoom
- UserInChatRoom
- Messages

#### Scalability and Performance (Optional)

> Please metion we are using RESTful HTTP APIs and asynchronous programming model.

### User Authentication Module

TODO: Yingying Liu

#### Basic Requirements

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

#### Technical Stack

TODO

### Instant Messaging Module

Our instant messaging (IM) module is responsible for communicating raw text messages between the different Clients in a Chat Room. We will use WebSockets to implement the real-time messaging module for our application, which is the standard practice in the industry to provide reliable and low-latency communication.

#### Basic Requirements

##### WebSocket Server

Our instant messaging app primarily deals with input/output operations, so we've designed the server to use asynchronous programming. This allows it to handle multiple user requests efficiently while maintaining fast response times. The Server will be built utilizing WebSocket and asynchronous tasks so that it will:

- Listen on a specified port for incoming connections
- Accept incoming connections from *multiple* Clients concurrently and handle each client in a separate asynchronous task
- Process incoming messages by enchoing them back to the client or broadcasting them to all Clients
- Split the WebSocket stream into a read and write half to handle bidirectional communication efficiently

##### WebSocket Client

Each Client will:

- Connect to the server via a WebSocket.
- Read messages from user inputs and send them to the Server during a live session.
- Receive and parse messages received from the Server and display them to the user.

#### Technical stack

The following technologies might be used in implementing the IM module:

- `Tokio`: An asynchronous runtime for Rust that facilitates concurrent operations and allows for efficient management of numerous connections.
- `Tokio-tungstenite`: A WebSocket library built on top of `Tokio`, allowing for easy integration of WebSockets with asynchronous code.
- `Futures`: A crate used to handle asynchronous streams and sinks, allowing efficient handling of real-time message flow between Clients and the Server.
  