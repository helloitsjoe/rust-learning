mod blackjack;

use blackjack::game::Game;
use blackjack::input::Input;

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

    // This is one way to do mock-able input.
    // Another way is to inject the io directly: https://stackoverflow.com/a/28370712/8852158
    let mut input = Input::new(Vec::new());
    let user_input = input.get_input();

    if user_input == "1" || user_input == "" {
        let mut game = Game::new(None);
        game.start(&mut input);
    } else {
        println!("You input: {}", user_input);
    }
}
