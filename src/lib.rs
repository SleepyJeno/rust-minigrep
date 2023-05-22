use std::{env};

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

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn check_args_provided(){
        use super::*;
        let query = "".to_string();
        let filename = "file.txt".to_string();
        let _config = Config::build(query.clone(), filename);
    }
}

