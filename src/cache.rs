use std::{path::{Path}, sync::{Mutex, Arc}};
use http::{Request, Response};
use crate::request::Error;
use super::{utils};


pub struct HTTPCache {
    dir_path: String,
    file_locks: Vec<Arc<Mutex<Option<String>>>>,
}

impl HTTPCache {
    pub fn new(file_name: &str) -> Self {
        Self {
            dir_path: file_name.to_string(),
            file_locks: vec![Arc::new(Mutex::new(None)); 997]
        }
    }

    pub fn contains_entry(&self, req: &Request<Vec<u8>>) -> bool {
        let hashcode = utils::get_hashcode(req);
        let path = format!("{}/{}", self.dir_path, hashcode);
        Path::new(&path).exists()
    }

    pub fn get_cached_response(&self, req: &Request<Vec<u8>>) -> Result<Response<Vec<u8>>, std::io::Error> {
        
    }

    pub fn add_entry(&self, req: &Request<Vec<u8>>, res: &Response<Vec<u8>>) {
        let hashcode = utils::get_hashcode(req);
        
    }
}