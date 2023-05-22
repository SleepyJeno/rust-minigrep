use std::{env, error, fs, io::{self, BufRead}};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn build(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Either the search string or the filename are missing");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config{
            query,
            filename
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents);

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn check_args_provided(){
        let args: Vec<String> = vec!["only-string".to_string()];
        let config = Config::build(&args[..]);
    }

    #[test]
    fn one_results() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

