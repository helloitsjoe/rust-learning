#[derive(std::fmt::Debug)]
pub struct Deck {
  cards: Vec<Card>,
}

impl Deck {
  pub fn new() -> Deck {
    let card_vals = (1..11).map(|val| Card { val }).collect::<Vec<Card>>();
    Deck {
      cards: Vec::from(card_vals),
    }
  }

  pub fn shuffle(&self) {
    println!("{:?}", &self.cards);
    println!("Shuffling");
  }

  pub fn deal_one(&self) {
    println!("Dealing one card");
  }
}

#[derive(std::fmt::Debug)]
pub struct Card {
  pub val: u32,
  // face: String,
}

struct Player {
  hand: Vec<Card>,
  total: u32,
  is_dealer: bool,
}

impl Player {
  pub fn new(is_dealer: bool) -> Player {
    Player {
      hand: Vec::new(),
      total: 0,
      is_dealer,
    }
  }

  pub fn deal(&self, deck: &Deck) {
    let player_or_dealer = {
      if self.is_dealer {
        "dealer"
      } else {
        "player"
      }
    };
    println!("Dealing {}", player_or_dealer);
    // self.hand.push(deck.deal_one());
    // self.hand.push(deck.deal_one());
    println!("{:?}", self.hand);
  }
}

pub struct Game {
  num_players: u32,
  player: Player,
  dealer: Player,
  deck: Deck,
}

impl Game {
  pub fn new(deck: Deck) -> Game {
    Game {
      num_players: 2,
      player: Player::new(false),
      dealer: Player::new(true),
      deck,
    }
  }

  pub fn start(&self) {
    println!("Let's play!");
    &self.deck.shuffle();
    &self.player.deal(&self.deck);
    &self.dealer.deal(&self.deck);
  }
}

#[cfg(test)]
mod test_blackjack {
  use super::*;

  #[test]
  fn test_construct() {
    let game = Game::new(Deck::new());
    game.start();
    assert_eq!(game.num_players, 2);
  }
}
