#[derive(Debug, Clone)]
pub struct Deck {
  pub cards: Vec<Card>,
  initial_size: u32,
}

fn make_cards(length: u32) -> Vec<Card> {
  let cards = (1..length + 1).map(|n| Card::new(n)).collect::<Vec<Card>>();
  Vec::from(cards)
}

impl Deck {
  pub fn new(default_cards: Option<Vec<Card>>) -> Deck {
    const DEFAULT_SIZE: u32 = 52;

    let cards = default_cards.unwrap_or(make_cards(DEFAULT_SIZE));
    let initial_size = cards.len() as u32;

    Deck {
      cards,
      initial_size,
    }
  }

  pub fn shuffle(self) {
    println!("{:?}", self.cards);
    println!("Shuffling");
  }

  pub fn deal_one(&mut self) -> Card {
    if self.cards.len() == 0 {
      self.cards = make_cards(self.initial_size);
    }

    self.cards.pop().unwrap()
  }
}

#[derive(Debug, Clone)]
pub struct Card {
  pub val: u32,
  face: String,
}

impl Card {
  pub fn new(num: u32) -> Card {
    let num_zero_indexed = num - 1;
    let val = (num_zero_indexed % 13) + 1;
    let val = if val > 10 { 10 } else { val };

    let face = match (num_zero_indexed % 52) + 1 {
      (1..=13) => String::from("D"),
      (14..=26) => String::from("H"),
      (15..=39) => String::from("C"),
      (16..=52) => String::from("S"),
      _ => String::from(""),
    };

    Card { val, face }
  }
}

#[test]
fn basic_deck() {
  let deck = Deck::new(None);
  assert_eq!(deck.cards.len(), 52);
  assert_eq!(deck.cards[0].val, 1);
  assert_eq!(deck.cards[12].val, 10);
  assert_eq!(deck.cards[13].val, 1);
  assert_eq!(deck.cards[25].val, 10);
  assert_eq!(deck.cards[26].val, 1);
  assert_eq!(deck.cards[38].val, 10);
  assert_eq!(deck.cards[39].val, 1);
  assert_eq!(deck.cards[51].val, 10);

  assert_eq!(deck.cards[0].face, String::from("D"));
  assert_eq!(deck.cards[12].face, String::from("D"));
  assert_eq!(deck.cards[13].face, String::from("H"));
  assert_eq!(deck.cards[25].face, String::from("H"));
  assert_eq!(deck.cards[26].face, String::from("C"));
  assert_eq!(deck.cards[38].face, String::from("C"));
  assert_eq!(deck.cards[39].face, String::from("S"));
  assert_eq!(deck.cards[51].face, String::from("S"));
}

#[test]
fn deck_deal_one() {
  let mut deck = Deck::new(Some(Vec::from([Card::new(2), Card::new(3)])));
  assert_eq!(deck.cards.len(), 2);
  deck.deal_one();
  assert_eq!(deck.cards.len(), 1);
}

#[test]
fn deck_deal_one_until_empty() {
  let mut deck = Deck::new(Some(Vec::from([Card::new(2), Card::new(3)])));
  deck.deal_one();
  assert_eq!(deck.cards.len(), 1);
  deck.deal_one();
  assert_eq!(deck.cards.len(), 0);
  deck.deal_one();
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
