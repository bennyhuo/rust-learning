use std::{env, process};

use ch12::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    match Config::new(&args) {
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
