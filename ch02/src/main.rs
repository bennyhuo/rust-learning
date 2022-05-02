use std::cmp::Ordering;
use std::io;
use std::process::exit;
use std::str::EscapeDebug;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    if cfg!(debug_assertions) {
        println!("The secret number is: {}", secret_number);
    }

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number, retry!!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // let guess_number = i32::from_str_radix(&guess.trim(), 10)
        //     .expect("Failed to convert to a number.");
        // if secret_number == guess_number {
        //     println!("You win!");
        // } else if secret_number < guess_number {
        //     println!("Too big!");
        // } else {
        //     println!("Too small!");
        // }
    }
}
