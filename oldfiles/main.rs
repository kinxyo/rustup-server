extern crate chrono;
use std::io::BufReader;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::SystemTime;
use chrono::{DateTime, Local};

fn main() {
    match File::open("log.txt") {
        Err(_) => {
            let mut file = File::create("log.txt").expect("permission denied while creating the file");
            file.write("Request Log\n".as_bytes()).expect("permission denied to write to the file");
            ()
        },
        Ok(_) => ()
    }

    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to listen to server");

    for stream in listener.incoming() {
        let stream = stream.unwrap(); //connection attempts

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    //now we're reading into the request
    let reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    println!("connection secured!");
    match std::fs::OpenOptions::new().append(true).open("log.txt") {
        Ok(mut file) => {
            let raw_time = SystemTime::now();
            let formatted_time: DateTime<Local> = raw_time.into();
            let time = format!("\nEntry of: {}\n",formatted_time.format("%d/%m/%Y %T"));
            file.write(time.as_bytes()).expect("failed to mark entry");
            for lines in http_request {
                file.write(lines.as_bytes()).expect("failed to write request log");
            }
            file.write("\n-----|\n".as_bytes()).expect("failed to close file");
        }
        Err(e) => println!("cannot write to log as: {e}"),
    }

    //now we'll write response in response to the request
    let status_line = "HTTP/1.1 200 OK";
    let contents = std::fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); //unable to create curly-brackets here
    stream.write_all(response.as_bytes()).unwrap();

}