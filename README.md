# RustChat

RustChat is a real-time message application written in Rust, using the [Axum](https://github.com/tokio-rs/axum) framework for backend and [Yew](https://yew.rs/) for frontend.

## Contributor Contacts 
|Worker | Student Number| Email|
|-------------|---------------|---------------|
|Yingying Liu |  1008325974   |lyy.liu@mail.utoronto.ca|
|Yang Hu       | 1005836794     | young.hu@mail.utoronto.ca|
|Ze Yang       | 1010007145   |zekevin.yang@mail.utoronto.ca|

## Motivation
Building a real-time chat application with Rust is driven by its exceptional performance, safety, and scalability.
Rust delivers C and C++-like performance, making it ideal for efficiently managing numerous simultaneous WebSocket connections, a critical aspect of real-time communication.
Its memory safety model, which operates without a garbage collector, ensures smooth and predictable performance—essential for maintaining responsiveness in such applications.
Rust’s ownership and borrowing system further enhances safe concurrency, eliminating risks like race conditions and data corruption when handling multiple users and chat rooms.
The async/await model provides robust asynchronous support, enabling the backend to scale seamlessly and manage thousands of concurrent users without system bottlenecks.
Leveraging libraries like Tokio for asynchronous programming and WebSocket crates simplifies the development of high-performance chat systems.
Rust’s full-stack potential is also unlocked through frameworks like Yew for front-end development, ensuring a unified and consistent codebase.
These strengths—high performance, safe concurrency, scalability, and a rich ecosystem—make Rust a compelling choice for building a reliable and scalable real-time chat application that prioritizes raw performance.
 
## Objectives & Features

Our objective is to build a high-performance real-time chat application designed for a seamless and straightforward user interaction.
The core features of our application include:

- Basic user authentication
- Create, join, and leave Chat Rooms
- Real-time communication powered by WebSocket
- Presence detection which notifies all users in a Chat Room when someone joins or leaves the room
- Message persistence: users will be able to view previous messages in a Chat Room
- A straightforward front-end web interface

## Reproducibility Guide

### Prerequisites

Before you start, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [MySQL](https://dev.mysql.com/downloads/mysql/)
- [Rust's WASM Target](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html)
- [Trunk](https://trunkrs.dev/)

If you are running on macOS and have `brew` installed, these tools can be installed via:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://sh.rustup.rs -sSf | sh
brew install mysql
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

### Clone the Repository

First, clone the repository to your local machine:

```sh
git clone https://github.com/huyang531/ece-1724-project.git
cd ece-1724-project
```

### Set Up the MySQL Database

1. Start MySQL service
    If you have installed MySQL using `Homebrew` on macOS, you can start the MySQL service via `Homebrew`:
   
    ```sh
    brew services start mysql
    ```
   
    Otherwise, please refer to [this guide](https://phoenixnap.com/kb/start-mysql-server) for launching the MySQL server on your platform.

1. Login to MySQL as the `root` user
   
    If you have just freshly installed MySQL, you should be able to login as `root` without a password:

    ```sh
    mysql -u root
    ```

    Otherwise, login using your existing password via:

    ```sh
    mysql -u root -p
    ```

1. Change the `root` password

    Our application assumes that the password to the root user is `root`.
    If that is not the case for you, please change the password using the following command:
    
    ```sql
    mysql> ALTER USER 'root'@'localhost' IDENTIFIED BY 'root';
    ```

1. Create the `chat_app` database

    ```sql
    DROP DATABASE IF EXISTS chat_app;
    CREATE DATABASE chat_app;
    ```
    > The first command will drop the `chat_app` database if it already exists - please proceed with caution.

### Clear the Ports

By default, the backend application binds to port `3000` and the frontend application binds to port `8080`.
If you have existing applications running on these ports, you need to kill them before running RustChat.
If you are running on a Unix-like system and are comfortable killing the processes running on these ports, you may do so by running this handly script:

```sh
./rust_chat_application/scripts/kill-ports.sh
```

### Build and Run the Backend

1. Navigate to the backend directory

    ```sh
    cd rust_chat_application
    ```

1. Build and run the backend application
   
    ```sh
    cargo run
    ```

### Build and Run the Backend

1. In another terminal, navigate to the frontend directory

    ```sh
    cd chatroom-yew
    ```

1. Build and run the frontend application

    ```sh
    trunk serve
    ```

## User's Guide

Once the backend are frontend servers are running, you may access the frontend at http://localhost:8080 via your browser.

### User Authentication

### Create Chat Room

### Join/Leave Chat Room


## Contributions by Each Team Member
|Worker | Contribution|
|-------------|---------------------|
|Yingying Liu |Implemented the User Authentication APIs at the backend|
|Yang Hu|Implemented the Frontend using Yew and the Messaging Module using Websocket|
|Ze Yang |Designed the software architecture, database schemas, and APIs; Implemented Chat-Room-related APIs at the backend|

## Lessons Learned and Concluding Remarks
Don't be a last-time person.

## Video Demo
The URL goes here.
