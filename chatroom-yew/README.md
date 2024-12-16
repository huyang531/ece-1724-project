# Chat Room in Yew

This is the frontend of a simple chat room application written in Rust using the Yew framework.

## Run

### Install WASM Target

This is needed to compile Rust code to WebAssembly.

```sh
rustup target add wasm32-unknown-unknown
```

### Install Trunk

Trunk is the recommended tool for managing deployment and packaging.

```sh
# note that this might take a while to install because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk
```

### Build and Run via Trunk

```sh
trunk serve
```
