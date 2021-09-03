use super::deck::{Card, Deck};

#[derive(Debug)]
pub enum PlayerState {
  Bust,
  Stand,
  Playing,
  Blackjack,
  // Won,
  Lost,
}

pub struct Player {
  pub hand: Vec<Card>,
  pub total: u32,
  is_dealer: bool,
  state: PlayerState,
}

impl Player {
  pub fn new(is_dealer: bool) -> Player {
    Player {
      hand: Vec::new(),
      total: 0,
      is_dealer,
      state: PlayerState::Playing,
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
    self.hand.push(card);
    self.total = self.score_cards(self.hand.clone());
    self.state = self.get_state(self.total);
  }

  pub fn play(&mut self, deck: &mut Deck) {
    // TODO: use traits to separate dealer/player behavior.
    // Only dealer should implement play()
    if self.total == 17 || self.total == 21 {
      self.state = PlayerState::Stand;
      return;
    }
    self.hit(deck.deal_one());
    if self.total > 21 {
      self.state = PlayerState::Bust;
      return;
    }
    self.play(deck);
  }

  fn score_cards(&self, cards: Vec<Card>) -> u32 {
    // TODO: Handle aces
    cards.into_iter().fold(0, |sum, card| card.val + sum)
  }

  pub fn get_state(&self, total: u32) -> PlayerState {
    match total {
      1..=20 => PlayerState::Playing,
      21 => PlayerState::Blackjack,
      _ => PlayerState::Lost,
    }
  }
}

#[test]
fn player_hit() {
  let mut player = Player::new(false);
  player.hit(Card::new(3));
  assert_eq!(player.hand.len(), 1);
  player.hit(Card::new(4));
  assert_eq!(player.hand.len(), 2);
  assert_eq!(player.total, 7);
}

#[test]
fn player_state() {
  let mut player = Player::new(false);
  player.hit(Card::new(10));
  player.hit(Card::new(10));
  assert!(matches!(player.state, PlayerState::Playing));
  player.hit(Card::new(1));
  assert!(matches!(player.state, PlayerState::Blackjack));
  player.hit(Card::new(1));
  assert!(matches!(player.state, PlayerState::Lost));
}

#[test]
fn dealer_stand_on_21() {
  let mut dealer = Player::new(true);
  let cards = Vec::from([Card::new(10), Card::new(6), Card::new(5)]);
  let mut deck = Deck::new(Some(cards));
  dealer.deal(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Playing));
  dealer.play(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Stand))
}

#[test]
fn dealer_hit_on_16() {
  let mut dealer = Player::new(true);
  let cards = Vec::from([Card::new(8), Card::new(8), Card::new(8)]);
  let mut deck = Deck::new(Some(cards));
  dealer.deal(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Playing));
  dealer.play(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Bust))
}

#[test]
fn dealer_stand_on_17() {
  let mut dealer = Player::new(true);
  let cards = Vec::from([Card::new(8), Card::new(9)]);
  let mut deck = Deck::new(Some(cards));
  dealer.deal(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Playing));
  dealer.play(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Stand))
}
