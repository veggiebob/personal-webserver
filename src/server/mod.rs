use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, usize};
use std::str::FromStr;
use std::sync::Arc;
use crate::info;
use crate::Logger;
use crate::server::gym_population::query_gym_data;
use crate::server::left_right_parse_demo::run_parse_demo;
use crate::server::Response::PlainText;
use crate::server::threadpool::ThreadPool;

mod threadpool;
mod cache;
pub mod left_right_parse_demo;
pub mod gym_population;
pub mod log;

pub fn main(site: Arc<Website>, address: &str) {
    info!("starting server...");
    let listener = TcpListener::bind(address).unwrap();
    let threadpool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let n_site = Arc::clone(&site);
        match stream {
            Ok(stream) => threadpool.execute(move || n_site.handle_connection(stream)),
            Err(e) => {
                info!("An error occurred when connecting to the client! Luckily, they'll probably try to connect again. {}", e);
            }
        }
    }
}

pub struct Website {
    loc: String
}

enum SendMethod {
    Binary,
    PlainText
}

enum Response {
    Binary(Vec<u8>),
    PlainText(String)
}

impl Website {
    pub fn new(website_location: String) -> Website {
        Website {
            loc: website_location
        }
    }
    fn get_resource(&self, url: String) -> Result<(SendMethod, String), String> {
        let path: Vec<&str> = url.split("/").into_iter().filter(|s| !s.is_empty()).collect();
        // println!("{:?}", path);
        if path.len() > 0 {
            let mut last_file = path.last().unwrap();
            let args: Vec<_> = last_file.split("?").collect();
            if args.len() > 1 {
                last_file = args.get(0).unwrap();
                let args: Vec<_> = args.last().unwrap().split("&").collect();
                // do something with args
            }
            if last_file.ends_with(".js") {
                Ok((SendMethod::PlainText, format!("{}/scripts/{}", self.loc, last_file)))
            } else if vec![".html", ".css"].iter().any(|s| last_file.ends_with(s)) {
                Ok((SendMethod::PlainText, format!("{}/layout/{}", self.loc, last_file)))
            } else if vec![".jpg", ".ico", ".png"].iter().any(|s| last_file.ends_with(s)) {
                Ok((SendMethod::Binary, format!("{}/layout/{}", self.loc, last_file)))
            } else {
                Err(format!("Don't know how to look for resource at {}", url))
            }
        } else {
            Ok((SendMethod::PlainText, format!("{}/layout/index.html", self.loc)))
        }
    }
    /**
    HTTP Format:
    ```
    data: [GET|SET|POST] URL HTTP/[HTTP Version]\r\n
    Header-Key: Header-Value\r\n
    ...
    Content-Length: [length in bytes]\r\n
    \r\n [notice double CRLF]
    [content with content length in bytes]
    ```
     */
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        if let Err(e) = stream.read(&mut buffer) {
            info!("An error occurred while trying to read from the string! Aborting! {}", e);
            return;
        }
        let data_as_string: String = String::from_utf8_lossy(&buffer[..]).into();
        // split into header and body
        let sections: Vec<_> = data_as_string.split("\r\n\r\n").collect();
        let response = if sections.len() >= 2 {

            let header = sections.get(0).unwrap();
            let header_data = parse_headers(header);

            let body = &buffer[(header.as_bytes().len() + 4)..]; // body as binary data
            let body_text = sections.get(1).unwrap();

            match header.split("\r\n").next() {
                Some(line) => {
                    let args = line.split(" ").collect::<Vec<_>>();
                    if args.len() < 3 {
                        create_bad_request_error("Badly formatted HTTP request.".to_string())
                    } else {
                        let message_type = args[0];
                        let url = args[1];
                        let http_version = args[2];
                        if http_version == "HTTP/6.9" {
                            Response::PlainText(format!("HTTP/6.9 420 nice 👌\r\n\r\n"))
                        } else {
                            match message_type {
                                "GET" => self.handle_get(url),
                                "POST" => {
                                    info!("received a POST message!");
                                    info!("data: {}", String::from_utf8_lossy(&buffer[..]));
                                    if let Some(len) = header_data.get("Content-Length") {
                                        if let Ok(len) = usize::from_str(len) {
                                            let body = &body[..len as usize];
                                            self.handle_put(url, &header_data, body)
                                        } else {
                                            create_bad_request_error("Content-Length not a number.".into())
                                        }
                                    } else {
                                        create_bad_request_error("PUT request missing Content-Length header.".into())
                                    }
                                },
                                _ => {
                                    create_bad_request_error("what are you even trying to do".to_string())
                                }
                            }
                        }
                    }
                },
                None => create_bad_request_error("Malformatted request.".to_string())
            }
        } else {
            create_bad_request_error("Malformatted request.".into())
        };
        match response {
            Response::PlainText(string) => {
                stream.write(string.as_bytes()).unwrap();
            },
            Response::Binary(data) => {
                stream.write(data.as_slice()).unwrap();
            }
        };
        stream.flush().unwrap();
    }

    fn handle_put(&self, url: &str, header: &Header, body: &[u8]) -> Response {
        // println!("url is {}", url);
        let body_text: String = String::from_utf8_lossy(body).into();
        if url == "/parse" {
            if let Some(mode) = header.get("Parse-Mode") {
                match run_parse_demo(body_text, mode, header.get("Output-Mode").unwrap_or(&"json".to_string())) {
                    Ok(output) => PlainText(output),
                    Err(e) => create_bad_request_error(e)
                }
            } else {
                create_bad_request_error("Parse requires a 'Parse-Mode' header to work.".into())
            }
        } else if url == "/gym-population" {
            // run the gym data demo!
            match query_gym_data("") {
                Ok(output) => PlainText(output),
                Err(e) => create_bad_request_error(e)
            }
        } else {
            create_bad_request_error(format!("Don't know what to do with the url {}", url))
        }
    }

    fn handle_get(&self, url: &str) -> Response {
        match self.get_resource(url.to_string()) {
            Ok((send_method, resource_path)) => match send_method {
                SendMethod::PlainText =>
                    match fs::read_to_string(resource_path.clone()) {
                        Ok(resource_file) => Response::PlainText(format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                            resource_file.len(),
                            resource_file
                        )),
                        Err(err) => create_bad_request_error(
                            format!("Cannot open file: {}", err.to_string())
                        )
                    },
                SendMethod::Binary =>
                    match fs::read(resource_path.clone()) {
                        Ok(binary_data) => {
                            let header = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                                binary_data.len());
                            let mut data = Vec::with_capacity(header.len() + binary_data.len());
                            for c in header.as_bytes() {
                                data.push(*c);
                            }
                            for b in binary_data {
                                data.push(b);
                            }
                            Response::Binary(data)
                        },
                        Err(err) => create_bad_request_error(
                            format!("Cannot open file: {}", err.to_string())
                        )
                    }
            },
            Err(error_message) => create_bad_request_error(
                format!("Cannot handle GET Request. {}", error_message))
        }
    }
}

/// Like JSON but without infinite depth!
type Header = HashMap<String, String>;

fn parse_headers<T: ToString>(header: T) -> Header {
    let header = header.to_string();
    let lines = header.split("\r\n");
    let mut data = Header::new();
    for line in lines {
        let parts = line.split(": ").collect::<Vec<_>>();
        if parts.len() == 2 {
            data.insert(
                parts.get(0).unwrap().trim().into(),
                parts.get(1).unwrap().trim().into()
            );
        }
    }
    data
}

fn create_bad_request_error(description: String) -> Response {
    Response::PlainText(format!("HTTP/1.1 400 {}\r\n\r\n", description))
}