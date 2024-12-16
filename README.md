# ECE-1724-project

## Contributor Contacts 
|Worker | Student Number| Email|
|-------------|---------------|---------------|
|Yingying Liu |  1008325974   |lyy.liu@mail.utoronto.ca|
|Yang Hu       | 1005836794     | ?|
|Ze Yang       | 1010007145   |zekevin.yang@mail.utoronto.ca|

## Motivation
Building a real-time chat application with Rust is driven by its exceptional performance, safety, and scalability. Rust delivers C and C++-like performance, making it ideal for efficiently managing numerous simultaneous WebSocket connections, a critical aspect of real-time communication. Its memory safety model, which operates without a garbage collector, ensures smooth and predictable performance—essential for maintaining responsiveness in such applications. Rust’s ownership and borrowing system further enhances safe concurrency, eliminating risks like race conditions and data corruption when handling multiple users and chat rooms. The async/await model provides robust asynchronous support, enabling the backend to scale seamlessly and manage thousands of concurrent users without system bottlenecks. Leveraging libraries like Tokio for asynchronous programming and WebSocket crates simplifies the development of high-performance chat systems. Rust’s full-stack potential is also unlocked through frameworks like Yew for front-end development, ensuring a unified and consistent codebase. These strengths—high performance, safe concurrency, scalability, and a rich ecosystem—make Rust a compelling choice for building a reliable and scalable real-time chat application that prioritizes raw performance.
 
## Objectives & Features
Create and implement a high-performance, scalable real-time chat application designed for instant messaging and seamless user interaction. The core features include basic user authentication, the ability to create and join chat rooms, and real-time communication powered by WebSockets for efficient data transmission. The application will also incorporate presence detection, enabling users to view who joined the chatroom and view the chat history. A straightforward front-end interface, implemented using a yew framework, will allow users to interact with the chat system. The primary focus will be on optimizing performance and ensuring the application scales effectively to handle growing user demand.


## Run RustChat Application

### Run Server

```sh
cd rust_chat_application
cargo run
```

### Ping Server with Clients

#### Using the `yew` Frontend (Which uses `tokio-tungstenite-wasm`)

Please refer to the front-end README for how to run the frontend.

#### Manually

We can ping the server using command line utilities or a web browser.
But first, we need to set up the server:

```sh
bash ./init.sh
```
This will create four users and three chat rooms.

Then, we can try communicate with the server using `wscat`, for example:

```sh
npm install -g wscat
wscat -c ws://localhost:3000/ws/1\?username=test\&user_id=99
```

## Run Example WS Application

### Run Server

```sh
cd ws_example_application
cargo run -p example-websockets --bin example-websockets
```

### Ping Server with Clients

#### Using `tokio-tungstenite` Client

```sh
cargo run -p example-websockets --bin example-client
```

Observe how the query parameters are captured succesfully and it can handle multiple client's requests.

#### Manually

We can ping the server using command line utilities or a web browser.
For example, we can communicate with the server using `wscat`:

```sh
npm install -g wscat
wscat -c ws://localhost:3011/ws/1
```


## Contributions by Each Team Member
|Worker | Contribution|
|-------------|---------------------|
|Yingying Liu |User Authentication APIs|
|Yang Hu|Frontend + Message Module + Websocket|
|Ze Yang |Database + Chatroom Functions + Backend Architecture|

## Lessons Learned and Concluding Remarks
Don't be a last-time person.

## Video Demo
The URL goes here.
