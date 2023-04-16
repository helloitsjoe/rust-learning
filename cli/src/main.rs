mod axum_server;
mod blackjack;
mod input;
mod mbta;
mod server;
mod tide_server;

use blackjack::game::Game;
use clap::Parser;
use input::Input;
use mbta::mbta::MBTA;

#[derive(Parser, Debug)]
struct Cli {
    arg: String,
}

// #[tokio::main]
#[async_std::main]
async fn main() {
    let args = Cli::try_parse().unwrap_or_else(|_| handle_user_input());
    let arg = args.arg;

    match arg.as_str() {
        "2" => {
            // axum_server::server::AxumServer::new(8080).await;
            tide_server::server::TideServer::start(8080).await;
            println!("Listening");
        }
        "3" => {
            // Fetch from MBTA
            let mut input = Input::new(Vec::new());
            let mbta = MBTA::new();
            mbta.start(&mut input).await;
        }
        _ => {
            // Blackjack for "1" and default
            let mut input = Input::new(Vec::new());
            let mut game = Game::new(None);
            game.start(&mut input);
        }
    }
}

fn handle_user_input() -> Cli {
    println!("What would you like to do?");
    println!("1 - Blackjack (default)");
    println!("2 - Server");
    println!("3 - MBTA");

    // This is one way to do mock-able input.
    // Another way is to inject the io directly: https://stackoverflow.com/a/28370712/8852158
    let mut input = Input::new(Vec::new());
    return Cli {
        arg: input.get_input(),
    };
}
