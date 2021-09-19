mod blackjack;
mod server;

use blackjack::game::Game;
use blackjack::input::Input;

use server::server::*;

// TODO: Use a CLI argument parser
fn main() {
    let mut args = std::env::args().skip(1);

    if args.len() == 0 {
        handle_user_input();
    } else {
        let arg = args.next().unwrap();
        if arg == "blackjack" {
            let mut input = Input::new(Vec::new());
            let mut game = Game::new(None);
            game.start(&mut input);
        } else if arg == "server" {
            let server = Server::new();
            server.listen(7878);
        } else {
            for arg in args {
                println!("{}", arg);
            }
            println!("Only blackjack for now!");
        }
    }
}

fn handle_user_input() {
    println!("What would you like to do?");
    println!("1 - Blackjack (default)");
    println!("2 - Server");

    // This is one way to do mock-able input.
    // Another way is to inject the io directly: https://stackoverflow.com/a/28370712/8852158
    let mut input = Input::new(Vec::new());
    let user_input = input.get_input();

    if user_input == "1" || user_input == "" {
        let mut game = Game::new(None);
        game.start(&mut input);
    } else if user_input == "2" {
        let server = Server::new();
        server.listen(7878);
    } else {
        println!("You input: {}", user_input);
    }
}
