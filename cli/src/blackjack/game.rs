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
    &mut self.deck.clone().shuffle();
    self.player.deal(&mut self.deck);
    self.dealer.deal(&mut self.deck);
    self.show_score();
    self.handle_input(input);
  }

  pub fn handle_input(&mut self, input: &mut Input) {
    println!("hit or stay? Type 'hit' or 'h' to hit, anything else to stay.");
    let lower = input.get_input().to_lowercase();
    match lower.as_str() {
      "hit" | "h" => {
        self.player.hit(self.deck.deal_one(true));
        self.show_score();

        // if matches!(
        //   self.player.get_state(self.player.total),
        //   PlayerState::Playing
        // ) {
        self.handle_input(input);
        // }
      }
      _ => {
        println!("You stayed.");
        self.show_score();
      // TODO: Play dealer
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
}

#[test]
fn new_game() {
  let deck = Deck::new(None);
  let game = &mut Game::new(Some(deck));
  game.start(&mut Input::new(Vec::from([String::from("stay")])));
  assert_eq!(game.num_players, 1);
  assert_eq!(game.player.hand.len(), 2);
  assert_eq!(game.dealer.hand.len(), 2);
  assert_eq!(game.deck.cards.len(), 48);
}

#[test]
fn player_hit() {
  let deck = Deck::new(None);
  let game = &mut Game::new(Some(deck));
  game.start(&mut Input::new(Vec::from([String::from("hit")])));
  assert_eq!(game.num_players, 1);
  assert_eq!(game.player.hand.len(), 3);
  assert_eq!(game.dealer.hand.len(), 2);
  assert_eq!(game.deck.cards.len(), 47);
}

#[test]
fn player_hit_twice() {
  let deck = Deck::new(None);
  let game = &mut Game::new(Some(deck));
  game.start(&mut Input::new(Vec::from([
    String::from("hit"),
    String::from("hit"),
  ])));
  assert_eq!(game.num_players, 1);
  assert_eq!(game.player.hand.len(), 4);
  assert_eq!(game.dealer.hand.len(), 2);
  assert_eq!(game.deck.cards.len(), 46);
}
