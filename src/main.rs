use minigrep::*;
use std::env;

fn main() {
    //the program accepts 2 positional arguments:
    //string to search for and a filename to search in 
    //optional parameter - IGNORE_CASE allows for case insensitive pattern matching

    //parse cli args and match as appropriate
    //iterate over a file and check for a partial match in EACH line
    //return all lines that contain a match and print

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    minigrep::run(config);

}
