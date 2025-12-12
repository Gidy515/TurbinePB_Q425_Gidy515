use std::env; use std::env::args;
use std::process; // process helps to exit the program without panicking 

use minigrep_project::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // collecting the command line arguments into a vector and printing them
    //dbg!(args); 

    // saving the argument values in variables
    //let query = &args[1];
    //let file_path = &args[2];

    //println!("looking for {query}");
    //println!("in file {file_path}");

    // Reading a file
    /*let contents = fs::read_to_string(file_path) // opens the file path and returns a std::io::Result<String> of the file's contents
                            .expect("Should have been able to read the file");
    println!("With text:\n{contents}");*/
    /*let (query, file_path) = parse_configs(&args);
    println!("looking for {query}");
    println!("in file {file_path}");*/
    
    /*let config = parse_configs(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);*/

    //let config = Config::new(&args);

    // Handling potential errors when creating a Config instance by using unwrap_or_else. unwrap_or_else takes a closure that will be executed if the Result is an Err value.
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    
    // Reading a file
    /*let contents = fs::read_to_string(config.file_path) // opens the file path and returns a std::io::Result<String> of the file's contents
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");*/

    //run(config);
    if let Err(e) = minigrep_project::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    
}

