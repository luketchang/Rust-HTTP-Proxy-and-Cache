use std::{collections::HashMap, net::SocketAddr};
use std::net::TcpStream;
use std::io::{Read, Write};
use http::{Request, Response, Method};
use http_bytes::{parse_request_header_easy};
use std::sync::Mutex;
use super::{cache::HTTPCache, strikeset::HTTPStrikeSet};

type HandlerFn = fn(&mut HTTPRequestHandler, req: Request<()>, stream: TcpStream);

pub struct HTTPRequestHandler {
    handlers: HashMap<Method, HandlerFn>,
    strikeset: HTTPStrikeSet,
    cache: Mutex<HTTPCache>,
}

impl HTTPRequestHandler {
    pub fn new() -> Self {
        let mut map: HashMap<Method, HandlerFn> = HashMap::new();
        map.insert(Method::GET, HTTPRequestHandler::handle_get);
        map.insert(Method::GET, HTTPRequestHandler::handle_post);

        Self {
            handlers: map,
            strikeset: HTTPStrikeSet{},
            cache: Mutex::new(HTTPCache{}),
        }
    }

    pub fn service_req(&self, mut stream: TcpStream) {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(_) => {
                if let Ok(Some((req, _))) = parse_request_header_easy(&buf) {
                    let method = req.method();
                    self.handlers[method](req, stream);
                } else {
                    println!("Failed to parse request: {}", String::from_utf8_lossy(&buf))
                }
            }
            Err(e) => println!("Failed to read from connection: {}", e)
        }
    }

    fn handle_get(&mut self, req: Request<()>, stream: TcpStream) {
        let ip_addr = stream.peer_addr().unwrap();
        let host = req.uri().unwrap().host();

    }

    fn handle_post(&mut self, req: Request<()>, stream: TcpStream) {

    }
}