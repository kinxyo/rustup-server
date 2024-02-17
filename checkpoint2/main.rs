extern crate chrono;
use std::io::BufReader;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::SystemTime;
use chrono::{DateTime, Local};

fn main() {
    
    check_files(); //creating log if doesn't exist.

    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to listen to server");

    for stream in listener.incoming() {
        let stream = stream.unwrap(); //connection attempts

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    
    /*
        now we're reading into the request
    
        we first listen to the incoming request at a port.
        then store it into buffer so we can read all at once for better performance.
        then we collect lines via iterator until iterator encounters an empty line (indicating the end of the HTTP request headers).
    */
    
    // READING REQUEST
    
    let reader = BufReader::new(&mut stream);
    
    let http_request: Vec<_> = reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();    
    
    println!("connection secured!");    
    log_request(&http_request);

    
    // SENDING RESPONSE
    
    const STATUS: &str = "HTTP/1.1 200 OK";
    let contents = std::fs::read_to_string("index.html").unwrap();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", STATUS, contents.len(), contents);

    stream.write_all(response.as_bytes()).unwrap();

    
    /*  
        So we're basically writing on the same tcpstream on which we recieved the request.

        When a client (like a web browser) makes a connection to the
        server, it sends the HTTP request over the TCP stream. 
        The server reads the request from the stream, processes it, 
        and then writes the HTTP response back to the same stream. 
        The client then reads the response from the stream.

    */

    
}


/* Logging Functions */

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