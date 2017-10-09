extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Collect the arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Unwrap the config type as either Config or Err
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Searching for: {}", config.query);
    // println!("In file: {}", config.filename);

    // Run the content scraper, with error handling
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
