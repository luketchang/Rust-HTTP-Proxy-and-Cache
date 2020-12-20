use http::{Request};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
fn it_works() {
    let mut req = Request::builder();
    println!("HASH {:?}", get_hashcode(&req.body(Vec::new()).unwrap()));
}