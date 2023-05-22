use minigrep::*;
use std::{env, process};

fn main() {
    //the program accepts 2 positional arguments:
    //string to search for and a filename to search in 
    //optional parameter - IGNORE_CASE allows for case insensitive pattern matching

    //parse cli args and match as appropriate
    //iterate over a file and check for a partial match in EACH line
    //return all lines that contain a match and print

    let mut args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Missing positional arguments, error: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Runtime error: {e}");
        process::exit(1);
    }

}
