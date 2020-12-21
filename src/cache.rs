use std::{fs::{File, copy, read, create_dir_all}, path::{Path}, sync::{Mutex, Arc}};
use std::io::prelude::*;
use http::{Request, Response};
use httparse;
use super::{utils, handler};
use super::http::{response};


pub struct HTTPCache {
    dir_path: String,
    file_locks: Vec<Arc<Mutex<()>>>,
}

impl HTTPCache {
    pub fn new(dir_path: &str) -> Self {
        create_dir_all(dir_path).unwrap();

        Self {
            dir_path: dir_path.to_string(),
            file_locks: vec![Arc::new(Mutex::new(())); 997]
        }
    }

    pub fn contains_entry(&self, req: &Request<Vec<u8>>) -> bool {
        let path = self.get_filepath_from_request(req);
        Path::new(&path).exists()
    }

    pub fn get_cached_response(&self, req: &Request<Vec<u8>>) -> Option<Response<Vec<u8>>> {
        if !Self::contains_entry(self, &req) {
            return None
        }

        let filepath = self.get_filepath_from_request(req);
        if let Ok(res_bytes) = read(&filepath) {
            if let Ok(Some((res, _))) = response::parse_response(&res_bytes) {
                return Some(res);
            }
            log::error!("Failed to parse response from cache file {}", &filepath);
            return None
        }
        log::error!("Failed to read response from cache file {}", &filepath);
        return None
    }

    pub fn add_entry(&self, req: &Request<Vec<u8>>, res: &Response<Vec<u8>>) {
        let filepath = Self::get_filepath_from_request(&self, req);
        let res_bytes = utils::response_to_bytes(res);

        if let Ok(mut file_buf) = File::create(&filepath) {
            match file_buf.write_all(&res_bytes) {
                Ok(_) => log::info!("Wrote file {} to cache.", &filepath),
                Err(e) => log::error!("Failed to write response to cache file: {}", e)
            }
        } else {
            log::error!("Failed to create new cache file!");
        }
    }

    fn get_filepath_from_request(&self, req: &Request<Vec<u8>>) -> String {
        let hashcode = utils::get_hashcode(req);
        format!("{}/{}", self.dir_path, hashcode)
    }
}

//TODO: create test mod to share imports and env logger
#[test]
fn adds_cache_file() {
    use http;

    let cache = HTTPCache::new(".");

    let req_builder = Request::builder()
        .uri("https://www.rust-lang.org/")
        .header("User-Agent", "my-awesome-agent/1.0");
    let req_vec: Vec<u8> = Vec::new();
    let request = req_builder.body(req_vec).unwrap();

    let res_builder = Response::builder()
        .header("Foo", "Bar")
        .status(http::StatusCode::OK);
    let res_vec: Vec<u8> = Vec::new();
    let response = res_builder.body(res_vec).unwrap();

    cache.add_entry(&request, &response);
    let expected_filepath = &cache.get_filepath_from_request(&request);
    assert!(Path::new(&expected_filepath).exists());
}

#[test]
fn retrieves_cache_file() {
    env_logger::init();

    let cache = HTTPCache::new(".");

    let req_builder = Request::builder()
        .uri("https://www.rust-lang.org/")
        .header("User-Agent", "my-awesome-agent/1.0");
    let req_vec: Vec<u8> = Vec::new();
    let request = req_builder.body(req_vec).unwrap();

    let res_builder = Response::builder()
        .header("Foo", "Bar")
        .status(http::StatusCode::OK);
    let res_vec: Vec<u8> = Vec::new();
    let response = res_builder.body(res_vec).unwrap();

    cache.add_entry(&request, &response);
    
    let retrieved_response = cache.get_cached_response(&request);
    println!("{:?}", retrieved_response);
    println!("{:?}", Some(response));
    //TODO: assert retrieved response == built response
}