use std::net;

fn main() {
    let msg = Message::Write("Hello".to_string());
    msg.call();

    let config_max: Option<u8> = None;//Some(3u8);
    let max = match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => ()
    };

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    } else {
        dbg!(config_max);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit")
            }
            Message::Move { x, y } => {
                println!("Move({x}, {y})")
            }
            Message::ChangeColor(_, _, blue) => {
                println!("blud: {blue}")
            }
            o => {
                dbg!(&o);
            }
        }
    }
}