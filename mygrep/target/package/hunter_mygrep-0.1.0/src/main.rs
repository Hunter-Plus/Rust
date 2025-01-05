use std::env;
use std::process;

use hunter_mygrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for string: {} In file path: {}",
        config.query, config.file_path
    );

    if let Err(e) = hunter_mygrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
