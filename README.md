# RustChat

RustChat is a real-time messaging application written in Rust, using the [Axum](https://github.com/tokio-rs/axum) framework for backend and [Yew](https://yew.rs/) for frontend.
It is the course project for [ECE1724 F1 Performant Software Systems with Rust](https://www.eecg.toronto.edu/~bli/ece1724) and this README also serves as the final project report.

## Contributor Contacts 
|Worker | Student Number| Email|
|-------------|---------------|---------------|
|Yingying Liu |  1008325974   |lyy.liu@mail.utoronto.ca|
|Yang Hu       | 1005836794     | young.hu@mail.utoronto.ca|
|Ze Yang       | 1010007145   |zekevin.yang@mail.utoronto.ca|

## Motivation

We are building a real-time messaging application in Rust driven by its exceptional performance, safety, and scalability promises.
Rust's memory safety model, which operates without a garbage collector, ensures smooth and predictable performance and its asynchronous programming model also makes it ideal for efficiently managing numerous simultaneous WebSocket connections — both are essential elements in a real-time chat application.

While Rust is most commonly used in system development, we explore Rust’s full-stack potential through frameworks like Yew (for front-end development) and Axum (for backend development), with the hope of building a reliable and scalable real-time chat application that prioritizes raw performance.
 
## Objectives & Features

Our objective is to build a high-performance real-time chat application designed for seamless and straightforward user interaction.
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

2. Login to MySQL as the `root` user
   
    If you have just freshly installed MySQL, you should be able to login as `root` without a password:

    ```sh
    mysql -u root
    ```

    Otherwise, login using your existing password via:

    ```sh
    mysql -u root -p
    ```

3. Change the `root` password

    Our application assumes that the password to the root user is `root`.
    If that is not the case for you, please change the password using the following command:
    
    ```sql
    mysql> ALTER USER 'root'@'localhost' IDENTIFIED BY 'root';
    ```

4. Create the `chat_app` database

    ```sql
    DROP DATABASE IF EXISTS chat_app;
    CREATE DATABASE chat_app;
    ```
    > The first command will drop the `chat_app` database if it already exists - please proceed with caution.

### Clear the Ports

By default, the backend application binds to port `3000` and the frontend application binds to port `8000`.
If you have existing applications running on these ports, you need to kill them before running RustChat.

If you are running on a Unix-like system and are comfortable killing the processes running on these ports, you may do so by running this handy script:

```sh
./rust_chat_application/scripts/kill-ports.sh
```

### Build and Run the Backend

1. Navigate to the backend directory

    ```sh
    cd rust_chat_application
    ```

2. Build and run the backend application
   
    ```sh
    cargo run
    ```

### Build and Run the Frontend

1. In another terminal, navigate to the frontend directory

    ```sh
    cd chatroom-yew
    ```

2. Build and run the frontend application

    ```sh
    trunk serve
    ```

## User's Guide

Once the backend and frontend servers are running, you may access the Home page at http://localhost:8000 via your browser.

<div align="center">
 <img width="1822" alt="image" src="https://github.com/user-attachments/assets/09a79a84-7eb3-40c4-ad2b-b365ea5cd1c3" />
 <p>Home Page</p>
</div>

### User Authentication

At the Home page, the user can log in or sign up by clicking the buttons on the top-right corner of the screen, where they will be warmly greeted by the following pages:

<div align="center">
 <img width="1822" alt="image" src="https://github.com/user-attachments/assets/841b5016-265e-446b-b813-caca6f530e1b" />
 <p>Sign Up Page</p>
</div>

<div align="center">
 <img width="1822" alt="image" src="https://github.com/user-attachments/assets/df799326-c414-4b03-9150-bf9761344d08" />
 <p>Login Page</p>
</div>

### Create/Join Chat Room

After logging in, the user can create a Chat Room on the left, and join an existing Chat Room via its ID on the right.

<div align="center">
 <img width="1822" alt="image" src="https://github.com/user-attachments/assets/2c537960-f724-44ea-9615-1452c8f00b5b" />
 <p>Home Page when logged-in</p>
</div>

On the successful creation of a Chat Room, the user will be notified of its ID via a browser prompt and navigated to the Chat Room created automatically.

<div align="center">
 <img width="322" alt="image" src="https://github.com/user-attachments/assets/e60b7189-dea8-41f2-9824-0547a0ec4eff" />
 <p>Browser prompt containing Chat Room ID</p>
</div>

If the user is trying to join a non-existing Chat Room, they will see the following error.

<div align="center">
 <img width="704" alt="image" src="https://github.com/user-attachments/assets/e62d87d3-28fb-4cd7-a601-a68519dc2a8c" />
 <p>Error</p>
</div>

### Chat!

The user can type their message in the text box at the bottom of the screen and press "Enter" or click "Send" to send it.

<div align="center">
 <img width="1822" alt="image" src="https://github.com/user-attachments/assets/451cebc3-c4ab-4401-835f-9fd7a68443ef" />
 <p>Chat Room Page</p>
</div>

As seen above, when a user joins or leaves the Chat Room, all users will be notified.
When a user enters a Chat Room, they will also be able to see all the previous messages exchanged in the chat.

### Leave Chat Room and Log Out

The user can leave the Chat Room by clicking the "Leave" button on the top-right corner of the Chat Room page.
This will lead the user to the Home page, where they could then log out by clicking "Logout“.

<div align="center">
 <img width="301" alt="image" src="https://github.com/user-attachments/assets/edda691e-dc4c-497b-81b8-e2f3b02ecc45" />
 <p>Successful log out message</p>
</div>

## Contributions by Each Team Member

|Worker | Contribution|
|-------------|---------------------|
|Yingying Liu |Implemented the User Authentication APIs at the backend|
|Yang Hu|Implemented the frontend using Yew and the Messaging Module using asynchronous WebSocket|
|Ze Yang |Designed the software architecture, database schemas, and APIs; Implemented Chat-Room-related APIs at the backend|

## Lessons Learned and Concluding Remarks

In this section, we share our key takeaways from this project and conclude the report by laying out some areas for future improvements.

### Key Takeaways

#### Rust is not fully mature yet.

Most of the crates we are using are still in their pre-release era, and even though they are  safe to use - as long as you can get them successfully compiled - there are sometimes significant flaws here and there.
For example,

- The `yew` framework dropped native support for WebSockets for some reason, which led to the creation of the `yew-websocket` crate

- The `tokio`-based crates do not provide native WASM support, so we had to resort to the third-party `tokio-tungstenite-wasm` for asynchronous WebSocket API support on the frontend.

Some of these crates have only limited documentation and a small community so it is hard to find help when running into problems.

Additionally, Rust's development toolchain is also a bit buggy.
We encountered compatibility issues with certain Rust crates on Windows and issues with the `rust-analyzer` false alarming us with syntax issues.
Our development experience highlighted the advantage of using macOS or Linux for Rust development, as it avoids the extra effort required to resolve system environment issues specific to Windows.

#### Don't build the wheels if you don't need to.

Considering the scope of this course project, we decided to build a simplistic User Authentication module from scratch rather than using a mature framework like JWT with advanced features such as session management.
However, contrary to our belief, the "advanced" features provided by JWT were actually necessary for ensuring the security and ease of use of the application, and rebuilding all these features from scratch - i.e., handling session management manually - proved to be tedious and error-prone.

#### Rust is safe, but time-consuming to develop.

The overall experience of developing with Rust was notably time-consuming, mostly due to the steep learning curve with concepts like ownership, mutable and immutable variables, and lifetimes.

However, as emphasized in the course, Rust development demands more time and effort during the design phase but significantly reduces debugging time after the application's release - indeed, we have not run into any memory-related bugs during development.

We believe that while the initial investment in design and development may seem burdensome, it ultimately leads to more reliable, performant, and maintainable software.
This makes Rust the perfect language for performance-critical and/or safety-critical systems, but for developing basic frontend/backend applications, it is debatable whether the runtime efficiency justifies the development cost.

### Areas for Improvement

- **Refactor user authentication APIs with JWT:** Implement JWT-based authentication to simplify session management and enhance security.

- **Containerize and deploy the application on the cloud:** Use Docker to containerize the application, enabling seamless deployment to cloud platforms.

- **Enhance UI design:** Improve the user interface to provide a more intuitive and visually appealing user experience.

## Video Demo

https://github.com/user-attachments/assets/b34a875a-b6f6-4fe0-86ec-c3ffd81bb364

## License

[MIT](https://github.com/huyang531/ece-1724-project/main/LICENSE)
