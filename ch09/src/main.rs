#![feature(result_flattening)]

use std::fs::{File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Read, Write};

fn main() {
    let file_name = "hello2.txt";

    let greeting_file_result = File::options().write(true).open(file_name);


    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!(".... {:?}", e),
            }
            other_error => {
                panic!("... {:?}", other_error)
            }
        },
    };

    let write_result = greeting_file.write("Hello World!!!!!".as_bytes());

    match write_result {
        Ok(size) => println!("written {size} bytes."),
        Err(error) => println!("Error: {error}."),
    }

    //
    //
    // if let Ok(mut file) = greeting_file_result {
    //     let mut str = String::new();
    //     let size = file.read_to_string(&mut str).unwrap_or(0);
    //     println!("{str}: {size}");
    // } else {
    //     println!("file not found");
    // }

    println!("username: {:?}", read_username_from_file());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt").and_then(|mut f| f.read_to_string(&mut username)
        .map(|_| username))
}