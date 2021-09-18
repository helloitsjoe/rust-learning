use super::deck::{Card, Deck};

#[derive(Debug)]
pub enum PlayerState {
  Bust,
  Stand,
  Playing,
  Blackjack,
  Won,
  Lost,
}

pub struct Player {
  pub hand: Vec<Card>,
  pub total: u32,
  is_dealer: bool,
  is_revealed: bool,
  state: PlayerState,
}

fn score_aces(sum: u32, card: &Card) -> u32 {
  let ace_over_21 = card.val == 11 && sum > 21;
  if ace_over_21 {
    sum - 10
  } else {
    sum
  }
}

impl Player {
  pub fn new(is_dealer: bool) -> Player {
    Player {
      hand: Vec::new(),
      total: 0,
      is_dealer,
      is_revealed: !is_dealer,
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
    self.hit(deck.deal_one(!self.is_dealer));
    self.hit(deck.deal_one(true));
    println!("{}", self.render_hand());
  }

  pub fn reveal(&mut self) {
    self.is_revealed = true;
    for card in &mut self.hand {
      card.reveal();
    }
  }

  pub fn hit(&mut self, card: Card) {
    self.hand.push(card);
    self.total = self.score_cards(&self.hand);
    // println!("Total: {}", self.total);
    self.state = self.get_state_from_total(self.total);
  }

  pub fn stand(&mut self) {
    self.state = PlayerState::Stand;
  }

  pub fn play(&mut self, deck: &mut Deck) {
    // TODO: use traits to separate dealer/player behavior.
    // Only dealer should implement play()

    if self.total > 16 && self.total <= 21 {
      self.stand();
      return;
    }
    self.hit(deck.deal_one(true));
    if self.total > 21 {
      self.state = PlayerState::Bust;
      return;
    }
    self.play(deck);
  }

  fn score_cards(&self, cards: &[Card]) -> u32 {
    let sum_aces_11 = cards.into_iter().fold(0, |sum, card| card.val + sum);

    let total = cards.into_iter().fold(sum_aces_11, score_aces);
    // println!("Total {}", total);
    total
  }

  pub fn render_hand(&self) -> String {
    let hand = &self.hand;
    hand
      .into_iter()
      .fold(String::new(), |s, card| s + &card.render() + "\n")
  }

  pub fn render_total(&self) -> String {
    if self.is_revealed {
      return self.total.to_string();
    }

    let hand = &self.hand;
    let sum = hand.into_iter().fold(
      0,
      |sum, card| {
        if card.face_up {
          sum + card.val
        } else {
          sum
        }
      },
    );

    sum.to_string()
  }

  fn get_state_from_total(&self, total: u32) -> PlayerState {
    match total {
      1..=20 => PlayerState::Playing,
      21 => PlayerState::Blackjack,
      _ => PlayerState::Bust,
    }
  }

  pub fn get_state(&self) -> &PlayerState {
    &self.state
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
  assert!(matches!(player.state, PlayerState::Bust));
}

#[test]
fn dealer_stand_on_21() {
  let mut dealer = Player::new(true);
  let cards = vec![10, 6, 5];
  let mut deck = Deck::new(Some(cards));
  dealer.deal(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Playing));
  dealer.play(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Stand))
}

#[test]
fn dealer_hit_on_16() {
  let mut dealer = Player::new(true);
  let cards = vec![8, 8, 8];
  let mut deck = Deck::new(Some(cards));
  dealer.deal(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Playing));
  dealer.play(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Bust))
}

#[test]
fn dealer_stand_on_17() {
  let mut dealer = Player::new(true);
  let cards = vec![8, 9];
  let mut deck = Deck::new(Some(cards));
  dealer.deal(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Playing));
  dealer.play(&mut deck);
  assert!(matches!(dealer.state, PlayerState::Stand))
}

#[test]
fn dealer_hide_and_reveal() {
  let mut dealer = Player::new(true);
  let cards = vec![8, 9];
  let mut deck = Deck::new(Some(cards));
  dealer.deal(&mut deck);
  assert_eq!(dealer.render_total(), "8");
  assert_eq!(dealer.render_hand(), "(Face down)\n8 of Diamonds\n");
  dealer.reveal();
  assert_eq!(dealer.render_total(), "17");
  assert_eq!(dealer.render_hand(), "9 of Diamonds\n8 of Diamonds\n");
}

#[test]
fn render_total_with_aces() {
  let mut dealer = Player::new(true);
  let cards = vec![9, 8, 1];
  let mut deck = Deck::new(Some(cards));
  dealer.deal(&mut deck);
  assert_eq!(dealer.render_total(), "8");
  dealer.hit(deck.deal_one(true));
  assert_eq!(dealer.render_total(), "17");
  dealer.reveal();
  assert_eq!(dealer.render_total(), "18");
}

#[test]
fn player_hand_is_reveaaled() {
  let mut player = Player::new(false);
  let cards = vec![8, 9];
  let mut deck = Deck::new(Some(cards));
  player.deal(&mut deck);
  assert_eq!(player.render_total(), "17");
  assert_eq!(player.render_hand(), "9 of Diamonds\n8 of Diamonds\n");
}

#[test]
fn ace() {
  let mut player = Player::new(false);
  // Cards pop from the end
  let cards = vec![1, 8, 1, 1];
  let mut deck = Deck::new(Some(cards));
  player.deal(&mut deck);
  assert_eq!(player.total, 12);
  player.hit(deck.deal_one(true));
  assert_eq!(player.total, 20);
  player.hit(deck.deal_one(true));
  assert_eq!(player.total, 21);
}

#[test]
fn ace_first() {
  let mut player = Player::new(false);
  // Cards pop from the end
  let cards = vec![10, 2, 1];
  let mut deck = Deck::new(Some(cards));
  player.deal(&mut deck);
  assert_eq!(player.total, 13);
  player.hit(deck.deal_one(true));
  assert_eq!(player.total, 13);
}

#[test]
fn multiple_aces() {
  let mut player = Player::new(false);
  // Cards pop from the end
  let cards = vec![10, 1, 1];
  let mut deck = Deck::new(Some(cards));
  player.deal(&mut deck);
  assert_eq!(player.total, 12);
  player.hit(deck.deal_one(true));
  assert_eq!(player.total, 12);
}
