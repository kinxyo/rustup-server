# rustup-server

A basic HTTP server written in Rust. It automatically finds the available port then listens for incoming connections, handles GET and POST requests, and logs the details of each request. The server uses the Tera templating engine to render HTML templates for the responses.

![image](https://github.com/kinxyo/rustup-server/assets/90744941/1ee81106-966b-4046-b9b6-74b23fc19f7a)

## Features

- Routing
- Path Parameter
- Automatic port selection if unavailable.
- Request tracking

## Architecture

The project is organized into three main files:

1. main.rs - The entry point of the application. It sets up the server and handles incoming connections.
2. endpoints.rs - Contains the logic for handling different types of HTTP requests and sending responses.
3. logfetch.rs - Responsible for logging incoming requests and fetching request data.

(stupid names, ik. will fix it later)

---

## Enhancements to add

Although I have completed what I intended to do, I may add the following enhancements if I find enough time:

- [ ] Include details about response in the log file.
- [ ] Allow the server's settings, such as the port number and log file location, to be configured through command-line arguments or a configuration file.
- [ ] Merge my cli-todolist with this project.
