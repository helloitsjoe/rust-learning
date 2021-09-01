use super::deck::{Card, Deck};

#[derive(Debug)]
pub enum PlayerState {
  Won,
  Lost,
  Playing,
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

  fn score_cards(&self, cards: Vec<Card>) -> u32 {
    // TODO: Handle aces
    cards.into_iter().fold(0, |sum, card| card.val + sum)
  }

  pub fn get_state(&self, total: u32) -> PlayerState {
    match total {
      1..=20 => PlayerState::Playing,
      21 => PlayerState::Won,
      _ => PlayerState::Lost,
    }
  }
}

#[cfg(test)]
mod test_player {
  use super::*;

  #[test]
  fn test_player_hit() {
    let mut player = Player::new(false);
    player.hit(Card::new(3));
    assert_eq!(player.hand.len(), 1);
    player.hit(Card::new(4));
    assert_eq!(player.hand.len(), 2);
    assert_eq!(player.total, 7);
  }

  #[test]
  fn test_player_state() {
    let mut player = Player::new(false);
    player.hit(Card::new(10));
    player.hit(Card::new(10));
    assert!(matches!(player.state, PlayerState::Playing));
    player.hit(Card::new(1));
    assert!(matches!(player.state, PlayerState::Won));
    player.hit(Card::new(1));
    assert!(matches!(player.state, PlayerState::Lost));
  }
}
