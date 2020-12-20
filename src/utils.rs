use http::{Request, Response};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::request;

pub fn response_to_bytes(req: &Response<Vec<u8>>) -> Vec<u8> {
    let serialized_req = format!("{:?}", req);
    serialized_req.into_bytes()
}

pub fn get_hashcode(req: &Request<Vec<u8>>) -> String {
    let serialized_req = format!("{:?}", req);
    calculate_hash(&serialized_req).to_string()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn calculates_request_hashcode() {
    const EXPECTED_HASH: &str = "12367140822854383472";

    let req_builder = Request::builder();
    let vec: Vec<u8> = Vec::new();
    let request = req_builder.body(vec).unwrap();
    let hashcode = get_hashcode(&request);
    assert_eq!(hashcode, EXPECTED_HASH)
}