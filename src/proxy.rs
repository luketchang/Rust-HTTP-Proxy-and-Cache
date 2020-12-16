use std::net::{TcpListener};
use std::sync::{Arc};
use super::scheduler::HTTPScheduler;

pub struct HTTPProxy {
    port_number: i64,
    scheduler: HTTPScheduler,
}

impl HTTPProxy {
    pub fn new(port_number: i64) -> Self {
        Self {
            port_number,
            scheduler: HTTPScheduler::new(),
        }
    }

    pub fn run(self) {
        let addr = format!("127.0.0.1:{}", self.port_number);
        let listener = TcpListener::bind(&addr).unwrap();
        println!("Listening on: {}", &addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => { self.scheduler.schedule_req(stream); },
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        };
    }
}