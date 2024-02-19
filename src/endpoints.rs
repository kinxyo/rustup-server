use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use std::fs::read_to_string;
use tera::{Context, Tera};

const STATUS: &str = "HTTP/1.1 200 OK";

pub async fn bind_to_available_port(starting_port: u16) -> Result<TcpListener, Box<dyn std::error::Error>> {
    let mut port = starting_port;
    loop {
        match TcpListener::bind(("127.0.0.1", port)).await {
            Ok(listener) => return Ok(listener),
            Err(_) => {
                port += 1;
                if port >= 65535 {
                    return Err("No available ports".into());
                }
            }
        }
    }
}

pub struct Variable {
    key: String,
    value: String,
}

/* Handling-Request Functions ⬇️ */

pub async fn get_req(path: &str, mut stream: &mut TcpStream) {
    match path {
        "/" => {
            send_response(&mut stream, "index", None).await;
        }

        _ => {
            send_response(&mut stream, "404", None).await;
        }
    }
}

pub async fn post_req(path: &str, stream: &mut TcpStream) {
    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() < 1 {
        println!("parts not enough!");
        return;
    }

    let parameter = parts[1];

    match path {
        _ if parameter.len() > 0 => {
            send_response(
                stream,
                "index",
                Some(Variable {
                    key: String::from("result"),
                    value: String::from(parts[1]),
                }),
            ).await;
        }
        _ => {
            send_response(stream, "404", None).await;
        }
    }
}

pub async fn unknown_req(stream: &mut TcpStream) {
    let contents = read_to_string("404.html").unwrap();
    let response = format!(
        // HTTP/1.1 405 Method Not Allowed\r\n\r\n
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        STATUS,
        contents.len(),
        contents
    );
    let _ = stream.write_all(response.as_bytes()).await;
}

pub async fn send_response(stream: &mut TcpStream, template: &str, variable: Option<Variable>) {
    match variable {
        Some(variable) => {
            let body = read_to_string(format!("{template}.html")).unwrap();

            let mut tera = Tera::default();
            tera.add_raw_template(template, &body).unwrap();

            let mut context = Context::new();
            context.insert(variable.key, &variable.value);

            let contents = tera.render(template, &context).unwrap();
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                STATUS,
                contents.len(),
                contents
            );
            let _ = stream.write_all(response.as_bytes()).await;
        }
        None => {
            let contents = read_to_string(format!("{template}.html")).unwrap();
            let mut tera = Tera::default();
            tera.add_raw_template(template, &contents).unwrap();
            let context = tera::Context::new(); // Create an empty context
            let contents = tera.render(template, &context).unwrap();
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                STATUS,
                contents.len(),
                contents
            );
            let _ = stream.write_all(response.as_bytes()).await;
        }
    }
}