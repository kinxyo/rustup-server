extern crate chrono;
use chrono::{DateTime, Local};
use std::time::SystemTime;
use std::io::{BufReader, Write, prelude::*};
use std::fs::{File, read_to_string};
use std::net::{TcpListener, TcpStream};
use tera::{Tera, Context};

const STATUS: &str = "HTTP/1.1 200 OK";

struct Variable {
    key: String,
    value: String,
}

fn main() {
    
    check_files(); //creating log if doesn't exist.

    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to listen to server");

    for stream in listener.incoming() {
        let stream = stream.unwrap(); //connection attempts

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    
    // READING REQUEST
    let reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();    
    
    let request_line = &http_request[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    println!("connection secured!");
    println!("{:?}", parts);
    log_request(&http_request);
    let method = parts[0];
    let path = parts[1];

    // let (method, path) = {
        // let mut iter = request.split_whitespace();
        // (iter.next().unwrap(), iter.next().unwrap())
    // };


    match method {
        "GET" => get_req(path, stream),
        "POST" => post_req(path, stream),
        _ => unknown_req(stream),
    }

    
}

/* Handling-Request Functions ⬇️ */

fn get_req(path: &str, stream: TcpStream) {

    match path {
        "/" => {
            send_response(stream, "index", None);
        },

        _ => {
            send_response(stream, "404", None);
        },
    }

}

fn post_req(path: &str, stream: TcpStream) {

    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() < 1 {
        println!("parts not enough!");
        return;
    }

    let parameter = parts[1];

    match path {
        _ if parameter.len() > 0 => {

            send_response(stream, "index", Some(Variable { key: String::from("result"), value: String::from(parts[1]) }));

        },
        _ => {
            send_response(stream, "404", None);
        }
    }

}

fn unknown_req(mut stream: TcpStream) {

    let contents = read_to_string("404.html").unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", STATUS, contents.len(), contents);
    stream.write_all(response.as_bytes()).unwrap();

}

/* Logging Functions ⬇️ */

fn check_files() {
    
    match File::open("log.txt") {
   
        Err(_) => {
            
            let mut file = File::create("log.txt").expect("permission denied while creating the file");
            file.write("Request Log\n".as_bytes()).expect("permission denied to write to the file");
            
            ()
        },
        
        Ok(_) => ()
   
    }
}

fn log_request(req: &Vec<String>) {
    
    match std::fs::OpenOptions::new().append(true).open("log.txt") {
        
        Ok(mut file) => {
            
            let raw_time = SystemTime::now();
            let formatted_time: DateTime<Local> = raw_time.into();
            let time = format!("\nEntry of: {}\n",formatted_time.format("%d/%m/%Y %T"));
            
            file.write(time.as_bytes()).expect("failed to mark entry");
            
            for lines in req {
                file.write(lines.as_bytes()).expect("failed to write request log");
            }
            
            file.write("\n-----|\n".as_bytes()).expect("failed to close file");
        }

    
        Err(e) => println!("cannot write to log as: {e}"),
    
    }
}

fn send_response(mut stream: TcpStream, template: &str, variable: Option<Variable>) {

    match variable {
        Some(variable) => {

            
            let body = read_to_string(format!("{template}.html")).unwrap();
            
            let mut tera = Tera::default();
            tera.add_raw_template(template, &body).unwrap();
            
            let mut context = Context::new();
            context.insert(variable.key, &variable.value);
            
            let contents = tera.render(template, &context).unwrap();          
            let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", STATUS, contents.len(), contents);
            stream.write_all(response.as_bytes()).unwrap();

        },
        None => {
            
            let contents = read_to_string(format!("{template}.html")).unwrap();          
            let mut tera = Tera::default();
            tera.add_raw_template(template, &contents).unwrap();
            let context = tera::Context::new(); // Create an empty context
            let contents = tera.render(template, &context).unwrap();
            let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", STATUS, contents.len(), contents);
            stream.write_all(response.as_bytes()).unwrap();    
        
        }
   
    }

}