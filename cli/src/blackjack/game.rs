use super::deck::Deck;
use super::input::Input;
use super::player::{Player, PlayerState};

pub struct Game {
  num_players: u32,
  player: Player,
  dealer: Player,
  deck: Deck,
}

impl Game {
  pub fn new(deck: Option<Deck>) -> Game {
    Game {
      num_players: 1,
      player: Player::new(false),
      dealer: Player::new(true),
      deck: deck.unwrap_or(Deck::new(None)),
    }
  }

  pub fn start(&mut self, input: &mut Input) {
    println!("Let's play! {} players", self.num_players);
    &mut self.deck.shuffle();
    // TODO: Should deal alternately to player/dealer
    self.player.deal(&mut self.deck);
    self.dealer.deal(&mut self.deck);
    self.show_score();
    self.handle_input(input);
  }

  pub fn handle_input(&mut self, input: &mut Input) {
    println!("hit or stay? Type 'hit' or 'h' to hit, anything else to stay.");
    match input.get_input().to_lowercase().as_str() {
      "hit" | "h" => {
        self.player.hit(self.deck.deal_one(true));
        println!("{}", self.player.render_hand());
        self.show_score();

        match self.player.get_state() {
          PlayerState::Bust => println!("Bust, you lose!"),
          PlayerState::Playing => self.handle_input(input),
          PlayerState::Blackjack => println!("Blackjack!"),
          _ => println!("Shouldn't get here"),
        }
      }
      _ => {
        println!("You stayed.");
        self.player.stand();
        self.dealer.reveal();
        self.dealer.play(&mut self.deck);
        println!("Dealer:\n{}", self.dealer.render_hand());
        self.show_score();
        let state = self.compare_player_dealer();

        match state {
          PlayerState::Bust => println!("Bust, you lose!"),
          PlayerState::Lost => println!("You lose!"),
          PlayerState::Stand => println!("Stand"),
          PlayerState::Won => println!("You win!"),
          PlayerState::Playing => println!("Shouldn't get here!"),
          PlayerState::Blackjack => println!("Blackjack!"),
        }
      }
    }
  }

  fn show_score(&self) {
    println!(
      "You have {}, dealer shows {}",
      self.player.render_total(),
      self.dealer.render_total()
    );
  }

  fn compare_player_dealer(&self) -> PlayerState {
    if self.dealer.total > 21 || (self.player.total <= 21 && self.player.total > self.dealer.total)
    {
      PlayerState::Won
    } else if self.dealer.total > self.player.total {
      PlayerState::Lost
    } else {
      self.player.get_state()
    }
  }
}

#[test]
fn new_game() {
  let deck = Deck::new(None);
  let game = &mut Game::new(Some(deck));
  game.start(&mut Input::new(Vec::from([String::from("stay")])));
  assert_eq!(game.num_players, 1);
  assert_eq!(game.player.hand.len(), 2);
  assert!(game.dealer.hand.len() >= 2);
  assert!(game.deck.cards.len() <= 48);
}

#[test]
fn player_hit() {
  let cards = vec![4, 3, 2, 2];
  let deck = Deck::new(Some(cards));
  let game = &mut Game::new(Some(deck));
  game.start(&mut Input::new(Vec::from([String::from("hit")])));
  assert_eq!(game.num_players, 1);
  assert_eq!(game.player.hand.len(), 3);
  assert!(game.dealer.hand.len() >= 2);
  assert!(game.deck.cards.len() <= 47);
}

#[test]
fn player_hit_twice() {
  let cards = vec![4, 3, 2, 2];
  let deck = Deck::new(Some(cards));
  let game = &mut Game::new(Some(deck));
  game.start(&mut Input::new(Vec::from([
    String::from("hit"),
    String::from("hit"),
  ])));
  assert_eq!(game.num_players, 1);
  assert_eq!(game.player.hand.len(), 4);
  assert!(game.dealer.hand.len() >= 2);
  assert!(game.deck.cards.len() <= 46);
}
