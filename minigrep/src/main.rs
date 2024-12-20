use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problme parsing arguments: {}", err);
        process::exit(1); // terminate the program with the status code passed
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


