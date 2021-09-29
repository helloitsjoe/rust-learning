mod blackjack;
mod server;

use blackjack::game::Game;
use blackjack::input::Input;

use server::server::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let arg = args.next().unwrap_or_else(|| handle_user_input());

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

fn handle_user_input() -> String {
    println!("What would you like to do?");
    println!("1 - Blackjack (default)");
    println!("2 - Server");

    // This is one way to do mock-able input.
    // Another way is to inject the io directly: https://stackoverflow.com/a/28370712/8852158
    let mut input = Input::new(Vec::new());
    let user_input = input.get_input();

    if user_input == "1" || user_input == "" {
        String::from("blackjack")
    } else if user_input == "2" {
        String::from("server")
    } else {
        println!("You input: {}", user_input);
        user_input
    }
}
