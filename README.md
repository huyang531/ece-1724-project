# ece-1724-project

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
For example, we can communicate with the server using `wscat`:

```sh
npm install -g wscat
wscat -c ws://localhost:3011/ws/1
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