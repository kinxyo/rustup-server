extern crate chrono;
use chrono::Local;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct RequestInfo {
    entry: String,
    request: String,
    #[serde(rename = "user-agent")]
    user_agent: String,
    connection: String,
    lang: String,
    host: String,
}

pub fn log_and_fetch_request(req: &String) -> String {

    let entry = Local::now().to_string();
    let mut request = String::new();
    let mut user_agent = String::new();
    let mut connection = String::new();
    let mut lang = String::new();
    let mut host = String::new();

    for line in req.lines() {
        let words: Vec<&str> = line.splitn(2, ": ").collect();
            match words[0] {
                "User-Agent" => user_agent = words[1].to_string(),
                "Connection" => connection = words[1].to_string(),
                "Accept-Language" => lang = words[1].to_string(),
                "Host" => host = words[1].to_string(),
                _ => {
                    let key = words[0].to_string();
                    if key.starts_with("GET") || key.starts_with("POST") {
                        request = key;
                    } 
                } 
            }
    }

    

    let data = RequestInfo {entry, request, user_agent, connection, lang, host};

    // println!("{:#?}", data);

    match log(&data) {
        Ok(_) => info!("new entry logged!"),
        Err(e) => info!("failed to log the request: {e}"),
    }

    data.request
}


// By dividing the function like this, fetching request will not be compromised by failing to log.
fn log(data: &RequestInfo) -> Result<(), Box<dyn std::error::Error>> {
    let records = to_string_pretty(&data)?;

    // println!("{}", records);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.ndjson")?;

        writeln!(file, "{}", records)?;

    Ok(())
}