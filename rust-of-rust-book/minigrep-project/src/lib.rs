use std::error::Error;
// Reading Argument values
use std::fs;

// The config struct shows that the query and file_path are connected
pub struct Config {
    query: String,
    file_path: String,
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

        Ok(Config { query, file_path })
    }
}

// Extracting the file reading logic into a separate function called run
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file"); 
    println!("With text:\n{contents}");

    Ok(())
}