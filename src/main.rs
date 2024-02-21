#![allow(dead_code)]

mod logfetch;
mod endpoints;

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use endpoints::{porting, handle_get, handle_post, unknown_req};
use logfetch::log_and_fetch_request;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;


#[tokio::main]
async fn main() {

    let connection_count = Arc::new(AtomicUsize::new(0));

    // simple_logger::init_with_level(Level::Info).unwrap(); //maybe ill use

    let listener = porting(3000).await.expect("failed to listen to server");
    
    let mut retry_count = 0;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let cc = Arc::clone(&connection_count);
                tokio::spawn(handle_connection(stream, cc)); //handling each connection concurrently (in the background).
                retry_count = 0; // reseting retry count on successful connection
            },
            Err(e) => {
                if retry_count < 5 {
                    println!("Failed to accept connection: {}, \nretrying...", e);
                    retry_count += 1;
                } else {
                    println!("Closing the program after failing to accept connection after 5 attempts:\nDebug: {}", e);
                    break;
                }
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, connection_count: Arc<AtomicUsize>) {

    /* Increasing Counter */
    connection_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    println!("New connection! Total: {}", connection_count.load(std::sync::atomic::Ordering::SeqCst));

    // Creating a buffer (BufReader is beyond me atm, it adds more complexity if anything).
    let mut buf = [0; 1024];

    /* Creating a loop to handle multiple request in 1 incoming connection */
    loop {

        let n = match stream.read(&mut buf).await {
            Ok(n) if n > 0 => n,
            Ok(_) => break,
            Err(e) => {eprintln!("failed to read data from the stream -> {:?}",e); return;}, //HANDLE THIS ERROR
        };

        let http_request = String::from_utf8_lossy(&buf[..n]).into_owned();
        let request = log_and_fetch_request(&http_request);
        let parts: Vec<&str> = request.split_whitespace().collect();

        let method = parts[0];
        let path = parts[1];

        /* Handling each request sequentially within a single connection */
        match method { 
            "GET" => handle_get(path, &mut stream).await,
            "POST" => handle_post(path, &mut stream).await,
            wrong_req => {
                println!("request failed for type: {wrong_req}");
                unknown_req(&mut stream).await
            },
        }

    }

    println!("A connection just closed!");
    connection_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    println!("Open Connections left: {}", connection_count.load(std::sync::atomic::Ordering::SeqCst));

}