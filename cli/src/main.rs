use std::env;
use std::io;

mod blackjack;
use blackjack::{Deck, Game};

// TODO: Use a CLI argument parser
fn main() {
    let args_with_fn = env::args().collect::<Vec<String>>();
    // Remove function from args
    let args = &args_with_fn[1..];

    if args.len() == 0 {
        handle_user_input();
    } else {
        for arg in args {
            println!("{}", arg);
            println!("Currently doesn't support args. Run this program without args.");
        }
    }
}

fn handle_user_input() {
    println!("What would you like to do?");
    println!("1 - Blackjack (default)");

    let input = get_input();

    if input == "1" || input == "" {
        let mut game = Game::new(Deck::new(None));
        game.start();
        let game_input = get_input();
        game.handle_input(game_input);
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
