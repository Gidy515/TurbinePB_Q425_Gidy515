use std::error::Error;
// Reading Argument values
use std::fs;
use std::env;

// The config struct shows that the query and file_path are connected
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/*fn parse_configs(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
} */
// Refactoring parse_config to return an instance of a Config struct

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> { // returning a result from Config::build
        // Adding a check for the number of arguments.
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case }) // returning an instance of Config
    }
}

pub fn search<'a>(
    query: &str, 
    contents: &'a str,
) -> Vec<&'a str> 
{
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
    //vec![]
    // Creating an empty vector to hold the results
}

// Extracting the file reading logic into a separate function called run
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the file
    let contents = fs::read_to_string(&config.file_path)?;

    // Decide which search function to use
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // Print all matched lines
    for line in results {
        println!("{line}");
    }

    Ok(())
}


pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

/*#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
}*/

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

assert_eq!(
    vec!["safe, fast, productive."],
    search(query, contents)
);
}

fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}


}