#[derive(std::fmt::Debug)]
pub struct Deck {
  cards: Vec<Card>,
  size: u32,
}

fn make_cards(length: u32) -> Vec<Card> {
  let card_vals = (1..length + 1)
    .map(|val| Card { val })
    .collect::<Vec<Card>>();
  Vec::from(card_vals)
}

impl Deck {
  pub fn new(default_size: Option<u32>) -> Deck {
    let size = default_size.unwrap_or(10);
    // TODO: Card values and faces
    Deck {
      cards: make_cards(size),
      size,
    }
  }

  pub fn shuffle(&self) {
    println!("{:?}", &self.cards);
    println!("Shuffling");
  }

  pub fn deal_one(&mut self) -> Card {
    println!("Dealing one card");
    if self.cards.len() > 0 {
      self.cards.pop().unwrap()
    } else {
      self.cards = make_cards(self.size);
      self.cards.pop().unwrap()
    }
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
    let game = Game::new(Deck::new(None));
    game.start();
    assert_eq!(game.num_players, 2);
  }

  #[test]
  fn test_deck_deal_one() {
    // Not clear to me why deck has to be declared as mutable here
    let mut deck = Deck::new(Some(2));
    let card = deck.deal_one();
    assert_eq!(deck.cards.len(), 1);
    assert_eq!(card.val, 2);
  }

  #[test]
  fn test_deck_deal_one_until_empty() {
    // Not clear to me why deck has to be declared as mutable here
    let mut deck = Deck::new(Some(2));
    let card = deck.deal_one();
    assert_eq!(deck.cards.len(), 1);
    assert_eq!(card.val, 2);
    let card = deck.deal_one();
    assert_eq!(deck.cards.len(), 0);
    assert_eq!(card.val, 1);
    let card = deck.deal_one();
    assert_eq!(deck.cards.len(), 1);
    assert_eq!(card.val, 2);
  }
}