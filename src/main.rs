use std::{env, thread::panicking};
use minigrep::*;

fn main() {
    //the program accepts 2 positional arguments:
    //string to search for and a filename to search in 
    //optional parameter - IGNORE_CASE allows for case insensitive pattern matching

    let query = "test".to_string();
    let filename = "file.txt".to_string();

    let config = Config::build(query, filename);

    println!("{:?} {:?}", config.query, config.filename);

    //parse cli args and match as appropriate
    //iterate over a file and check for a partial match in EACH line
    //return all lines that contain a match and print
}
