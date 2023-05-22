use std::{env, thread::panicking};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn build(query: String, filename: String) -> Config {
        if env::args().len() < 3 {
            panic!("Either the search string or the filename are missing");
        }
        Config{
            query,
            filename
        }
    }
}

