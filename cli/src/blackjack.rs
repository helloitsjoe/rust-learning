#[derive(Debug, Clone)]
pub struct Deck {
  cards: Vec<Card>,
  initial_size: u32,
}

fn make_cards(length: u32) -> Vec<Card> {
  let card_vals = (1..length + 1)
    .map(|val| Card::new(val))
    .collect::<Vec<Card>>();
  Vec::from(card_vals)
}

impl Deck {
  pub fn new(default_size: Option<u32>) -> Deck {
    let initial_size = default_size.unwrap_or(10);

    Deck {
      cards: make_cards(initial_size),
      initial_size,
    }
  }

  pub fn shuffle(self) {
    println!("{:?}", self.cards);
    println!("Shuffling");
  }

  pub fn deal_one(&mut self) -> Card {
    println!("Dealing one card");
    if self.cards.len() == 0 {
      self.cards = make_cards(self.initial_size);
    }

    self.cards.pop().unwrap()
  }
}

#[derive(Debug, Clone)]
pub struct Card {
  pub val: u32,
  // face: String,
}

impl Card {
  pub fn new(val: u32) -> Card {
    Card { val }
  }
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

  pub fn deal(&mut self, deck: &mut Deck) {
    let player_or_dealer = {
      if self.is_dealer {
        "dealer"
      } else {
        "player"
      }
    };
    println!("Dealing {}", player_or_dealer);
    self.hit(deck.deal_one());
    self.hit(deck.deal_one());
    println!("{:?}", self.hand);
  }

  pub fn hit(&mut self, card: Card) {
    self.total += card.val;
    self.hand.push(card);
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

  pub fn start(&mut self) {
    println!("Let's play!");
    &mut self.deck.clone().shuffle();
    self.player.deal(&mut self.deck);
    self.dealer.deal(&mut self.deck);
  }
}

#[cfg(test)]
mod test_blackjack {
  use super::*;

  #[test]
  fn test_construct() {
    let game = &mut Game::new(Deck::new(Some(10)));
    game.start();
    assert_eq!(game.num_players, 2);
    assert_eq!(game.player.hand.len(), 2);
    assert_eq!(game.dealer.hand.len(), 2);
    assert_eq!(game.deck.cards.len(), 6);
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

  #[test]
  fn test_player_hit() {
    let mut player = Player::new(false);
    player.hit(Card::new(3));
    assert_eq!(player.hand.len(), 1);
    player.hit(Card::new(4));
    assert_eq!(player.hand.len(), 2);
    assert_eq!(player.total, 7);
  }
}
