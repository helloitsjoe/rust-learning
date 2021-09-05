use super::deck::Deck;
use super::player::Player;

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

  pub fn start(&mut self) {
    println!("Let's play! {} players", self.num_players);
    &mut self.deck.clone().shuffle();
    self.player.deal(&mut self.deck);
    self.dealer.deal(&mut self.deck);
    self.show_score();
    println!("hit or stay?");
  }

  pub fn handle_input(&mut self, input: String) {
    if input == "hit" {
      self.player.hit(self.deck.deal_one(true));
      self.show_score();
    } else {
      println!("You stayed.");
      self.show_score();
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
  game.start();
  assert_eq!(game.num_players, 1);
  assert_eq!(game.player.hand.len(), 2);
  assert_eq!(game.dealer.hand.len(), 2);
  assert_eq!(game.deck.cards.len(), 48);
}
