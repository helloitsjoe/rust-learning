mod blackjack;
mod server;

use blackjack::game::Game;
use blackjack::input::Input;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    arg: String,
}

use server::server::*;

fn main() {
    let args = Cli::try_parse().unwrap_or_else(|_| handle_user_input());
    let arg = args.arg;

    if arg == "blackjack" {
        let mut input = Input::new(Vec::new());
        let mut game = Game::new(None);
        game.start(&mut input);
    } else if arg == "server" {
        let server = Server::new();
        server.listen(7878);
    } else {
        println!("Only blackjack and server for now!");
    }
}

fn handle_user_input() -> Cli {
    println!("What would you like to do?");
    println!("1 - Blackjack (default)");
    println!("2 - Server");

    // This is one way to do mock-able input.
    // Another way is to inject the io directly: https://stackoverflow.com/a/28370712/8852158
    let mut input = Input::new(Vec::new());
    let user_input = input.get_input();

    if user_input == "1" || user_input == "" {
        Cli {
            arg: "blackjack".to_string(),
        }
    } else if user_input == "2" {
        Cli {
            arg: "server".to_string(),
        }
    } else {
        println!("You input: {}", user_input);
        Cli { arg: user_input }
    }
}
