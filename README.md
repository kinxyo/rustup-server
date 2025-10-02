# rustup-server

A basic HTTP server written in Rust. 

![image](https://github.com/kinxyo/rustup-server/assets/90744941/1ee81106-966b-4046-b9b6-74b23fc19f7a)

## what works

- Routing
- Path Parameter
- Automatic port selection if unavailable.
- Request tracking

## directory

The directory is organized into three main files:

1. main.rs - The entry point of the application. It sets up the server and handles incoming connections.
2. endpoints.rs - Contains the logic for handling different types of HTTP requests and sending responses.
3. logfetch.rs - Responsible for logging incoming requests and fetching request data.

(stupid names, ik. will fix it later)

---
