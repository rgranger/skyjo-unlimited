use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Game {
    first_player: Option<u8>,
    players: Vec<Player>,
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Game {
    pub fn new(player_count: u8) -> Game {
        assert!((2..=8).contains(&player_count));

        Game {
            first_player: None,
            players: (0..player_count).map(|_| Player::new()).collect(),
            draw_pile: create_deck(),
            discard_pile: Vec::new(),
        }
    }
}

fn create_deck() -> Vec<Card> {
    let mut cards = Vec::new();
    (0..5).for_each(|_| cards.push(Card::new(-2)));
    (0..10).for_each(|_| cards.push(Card::new(-1)));
    (0..15).for_each(|_| cards.push(Card::new(0)));
    (0..10).for_each(|_| cards.push(Card::new(1)));
    (0..10).for_each(|_| cards.push(Card::new(2)));
    (0..10).for_each(|_| cards.push(Card::new(3)));
    (0..10).for_each(|_| cards.push(Card::new(4)));
    (0..10).for_each(|_| cards.push(Card::new(5)));
    (0..10).for_each(|_| cards.push(Card::new(6)));
    (0..10).for_each(|_| cards.push(Card::new(7)));
    (0..10).for_each(|_| cards.push(Card::new(8)));
    (0..10).for_each(|_| cards.push(Card::new(9)));
    (0..10).for_each(|_| cards.push(Card::new(10)));
    (0..10).for_each(|_| cards.push(Card::new(11)));
    (0..10).for_each(|_| cards.push(Card::new(12)));

    cards
}

#[derive(Debug)]
struct Player {
    score: u16,
    cards: PlayerCards,
}

impl Player {
    fn new() -> Player {
        Player {
            score: 0,
            cards: [None, None, None, None],
        }
    }
}

type PlayerCards = [Option<[Card; 3]>; 4];

#[derive(Debug)]
struct Card {
    value: i8,
    status: CardStatus,
}

impl Card {
    fn new(value: i8) -> Card {
        assert!((-2..=12).contains(&value));

        Card {
            value,
            status: CardStatus::Hidden,
        }
    }
}

#[derive(Debug)]
enum CardStatus {
    Shown,
    Hidden,
}

pub enum PlayerDeck {
    Slot_1_1,
    Slot_1_2,
    Slot_1_3,
    Slot_2_1,
    Slot_2_2,
    Slot_2_3,
    Slot_3_1,
    Slot_3_2,
    Slot_3_3,
    Slot_4_1,
    Slot_4_2,
    Slot_4_3,
}
