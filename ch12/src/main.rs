use std::{env, process};

use ch12::Config;

fn main() {
    match Config::new(env::args()) {
        Ok(config) => {
            if let Err(e) = ch12::run(config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
        Err(message) => {
            eprintln!("Problem parsing arguments: {message}");
        }
    }
}
