use std::net::{TcpListener};
use super::handler::HTTPRequestHandler;

pub struct HTTPProxy {
    port_number: i64,
    handler: HTTPRequestHandler,
}

impl HTTPProxy {
    pub fn new(port_number: i64) -> Self {
        Self {
            port_number,
            handler: HTTPRequestHandler::new(),
        }
    }

    pub fn run(self) {
        let addr = format!("127.0.0.1:{}", self.port_number);
        let listener = TcpListener::bind(&addr).unwrap();
        println!("Listening on: {}", &addr);

        loop {
            match listener.accept() {
                Ok((mut client_conn, _)) => { &self.handler.service_req(client_conn); },
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        };
    }
}