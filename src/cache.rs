use std::{collections::HashSet, path::{PathBuf}};

pub struct HTTPCache {
    file_path: PathBuf,
}

impl HTTPCache {
    pub fn new(file_name: &str) -> Self {
        //TODO: create/replace file with given name
        Self {
            file_path: PathBuf::from(file_name)
        }
    }
}