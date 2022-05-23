use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        // process::exit(1) doesn't work here (Windows issue?)
        process::exit(0);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(0);
    }
}