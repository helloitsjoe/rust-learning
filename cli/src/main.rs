use std::collections::VecDeque;
use std::env;
use std::io;

// TODO: Use a CLI argument parser
fn main() {
    let mut args: VecDeque<String> = env::args().collect();
    // Remove filename from args
    args.drain(..1);

    if args.len() == 0 {
        handle_user_input();
    } else {
        for arg in args {
            println!("{}", arg);
        }
    }
}

fn handle_user_input() {
    println!("What would you like to do?");
    println!("1 - Blackjack");

    let input = get_input();
    if input == "1" {
        println!("Blackjack!");
    } else {
        println!("You input: {}", input);
    }
}

fn get_input() -> String {
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Something went wrong");

    String::from(buf.trim())
}
