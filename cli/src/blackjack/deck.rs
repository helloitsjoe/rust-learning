use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
  pub cards: Vec<Card>,
  initial_values: Vec<u32>,
}

fn make_cards(nums: &[u32]) -> Vec<Card> {
  nums.iter().map(|&n| Card::new(n)).collect::<Vec<Card>>()
}

impl Deck {
  pub fn new(default_values: Option<Vec<u32>>) -> Deck {
    let initial_values = (1..53).collect::<Vec<u32>>();
    let values = default_values.unwrap_or(initial_values);

    Deck {
      cards: make_cards(&values),
      initial_values: values,
    }
  }

  pub fn shuffle(&mut self) {
    println!("Shuffling");
    self.cards.shuffle(&mut thread_rng());
    // println!("{:?}", self.cards);
  }

  pub fn deal_one(&mut self, face_up: bool) -> Card {
    let mut card = self.cards.pop().unwrap_or_else(|| {
      println!("Shuffling new deck...");
      self.cards = make_cards(&self.initial_values);
      self.cards.pop().expect("cards should never be empty")
    });

    if face_up {
      card.reveal();
    }

    card
  }
}

#[derive(Debug)]
pub struct Card {
  pub val: u32,
  face: String,
  suit: String,
  pub face_up: bool,
}

impl Card {
  pub fn new(num: u32) -> Card {
    let num_zero_indexed = num - 1;
    let val = (num_zero_indexed % 13) + 1;
    let val_aces_one = if val > 10 { 10 } else { val };
    let val = if val_aces_one == 1 { 11 } else { val_aces_one };

    let face = match (num_zero_indexed % 13) + 1 {
      11 => String::from("J"),
      12 => String::from("Q"),
      13 => String::from("K"),
      1 => String::from("A"),
      n => n.to_string(),
    };

    let suit = match (num_zero_indexed % 52) + 1 {
      (1..=13) => String::from("Diamonds"),
      (14..=26) => String::from("Hearts"),
      (15..=39) => String::from("Clubs"),
      (16..=52) => String::from("Spades"),
      _ => String::from(""),
    };

    Card {
      val,
      face,
      suit,
      face_up: false,
    }
  }

  pub fn render(&self) -> String {
    if self.face_up {
      format!("{} of {}", self.face, self.suit)
    } else {
      format!("(Face down)")
    }
  }

  pub fn reveal(&mut self) {
    self.face_up = true;
  }
}

#[test]
fn basic_deck() {
  let deck = Deck::new(None);
  assert_eq!(deck.cards.len(), 52);

  // Make sure face cards are 10
  assert_eq!(deck.cards[0].val, 11);
  assert_eq!(deck.cards[9].val, 10);
  assert_eq!(deck.cards[10].val, 10);
  assert_eq!(deck.cards[11].val, 10);
  assert_eq!(deck.cards[12].val, 10);
  // Make sure values loop from 10 back to 1
  assert_eq!(deck.cards[13].val, 11);
  assert_eq!(deck.cards[25].val, 10);
  assert_eq!(deck.cards[26].val, 11);
  assert_eq!(deck.cards[38].val, 10);
  assert_eq!(deck.cards[39].val, 11);
  assert_eq!(deck.cards[51].val, 10);

  // Make sure faces loop from K back to A
  assert_eq!(deck.cards[0].face, "A");
  assert_eq!(deck.cards[12].face, "K");
  assert_eq!(deck.cards[13].face, "A");
  assert_eq!(deck.cards[25].face, "K");
  assert_eq!(deck.cards[26].face, "A");
  assert_eq!(deck.cards[38].face, "K");
  assert_eq!(deck.cards[39].face, "A");
  assert_eq!(deck.cards[51].face, "K");

  assert_eq!(deck.cards[0].suit, String::from("Diamonds"));
  assert_eq!(deck.cards[12].suit, String::from("Diamonds"));
  assert_eq!(deck.cards[13].suit, String::from("Hearts"));
  assert_eq!(deck.cards[25].suit, String::from("Hearts"));
  assert_eq!(deck.cards[26].suit, String::from("Clubs"));
  assert_eq!(deck.cards[38].suit, String::from("Clubs"));
  assert_eq!(deck.cards[39].suit, String::from("Spades"));
  assert_eq!(deck.cards[51].suit, String::from("Spades"));
}

#[test]
fn deck_deal_one() {
  let mut deck = Deck::new(Some(vec![2, 3]));
  assert_eq!(deck.cards.len(), 2);
  deck.deal_one(true);
  assert_eq!(deck.cards.len(), 1);
}

#[test]
fn deck_deal_one_until_empty() {
  let mut deck = Deck::new(Some(vec![2, 3]));
  deck.deal_one(true);
  assert_eq!(deck.cards.len(), 1);
  deck.deal_one(true);
  assert_eq!(deck.cards.len(), 0);
  deck.deal_one(true);
  assert_eq!(deck.cards.len(), 1);
}

#[test]
fn card_val() {
  let card = Card::new(2);
  assert_eq!(card.val, 2);
  let card = Card::new(10);
  assert_eq!(card.val, 10);
  let card = Card::new(11);
  assert_eq!(card.val, 10);
  let card = Card::new(12);
  assert_eq!(card.val, 10);
  let card = Card::new(13);
  assert_eq!(card.val, 10);
}

#[test]
fn card_reveal() {
  let mut card = Card::new(2);
  assert_eq!(card.render(), "(Face down)");
  card.reveal();
  assert_eq!(card.render(), "2 of Diamonds");
}

// This has a small chance of failing
#[test]
fn shuffle() {
  let mut deck = Deck::new(None);
  let cards = &deck.cards;
  assert!(cards[0].val == 11 && cards[1].val == 2 && cards[2].val == 3);
  deck.shuffle();
  let cards = &deck.cards;
  assert!(!(cards[0].val == 11 && cards[1].val == 2 && cards[2].val == 3));
}
