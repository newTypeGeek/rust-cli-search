use std::env;
use std::process;

use rust_cli_search::Config;

fn main() {
    // read CLI arguments
    let args: Vec<String> = env::args().collect();

    // create Config instance
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // run the search program
    if let Err(e) = rust_cli_search::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
