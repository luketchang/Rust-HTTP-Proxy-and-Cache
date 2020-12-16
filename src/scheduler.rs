use threadpool::ThreadPool;
use std::net::{ TcpStream };
use std::sync::{Arc, Mutex};

use super::handler::{HTTPRequestHandler};
use super::strikeset::{HTTPStrikeSet};
use super::cache::{HTTPCache};

pub struct HTTPScheduler {
    pool: ThreadPool,
    request_handler: Arc<HTTPRequestHandler>,
}

impl HTTPScheduler {
    pub fn new() -> Self {
        Self {
            pool: ThreadPool::new(64),
            request_handler: Arc::new(HTTPRequestHandler::new()),
        }
    }

    pub fn schedule_req(&self, mut stream: TcpStream) {
        let handler = Arc::clone(&self.request_handler);
        &self.pool.execute(move || { handler.service_req(stream) });
    }
}