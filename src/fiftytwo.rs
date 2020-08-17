use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub mod old_maid;

#[derive(Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Clubs    => write!(f, "{}", "♣"),
            Suit::Diamonds => write!(f, "{}", "♦"),
            Suit::Hearts   => write!(f, "{}", "♥"),
            Suit::Spades   => write!(f, "{}", "♠"),
        }
    }
}

#[derive(Clone)]
pub enum CardTitle {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for CardTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardTitle::Ace   => write!(f, "{}", "A"),
            CardTitle::Two   => write!(f, "{}", "2"),
            CardTitle::Three => write!(f, "{}", "3"),
            CardTitle::Four  => write!(f, "{}", "4"),
            CardTitle::Five  => write!(f, "{}", "5"),
            CardTitle::Six   => write!(f, "{}", "6"),
            CardTitle::Seven => write!(f, "{}", "7"),
            CardTitle::Eight => write!(f, "{}", "8"),
            CardTitle::Nine  => write!(f, "{}", "9"),
            CardTitle::Ten   => write!(f, "{}", "10"),
            CardTitle::Jack  => write!(f, "{}", "J"),
            CardTitle::King  => write!(f, "{}", "K"),
            CardTitle::Queen => write!(f, "{}", "Q"),
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
    pub title: CardTitle
}

impl Card {
    pub fn new(value: u8, suit: Suit) -> Card {
        match value {
            1  => Card { suit, value, title: CardTitle::Ace },
            2  => Card { suit, value, title: CardTitle::Two },
            3  => Card { suit, value, title: CardTitle::Three },
            4  => Card { suit, value, title: CardTitle::Four },
            5  => Card { suit, value, title: CardTitle::Five },
            6  => Card { suit, value, title: CardTitle::Six },
            7  => Card { suit, value, title: CardTitle::Seven },
            8  => Card { suit, value, title: CardTitle::Eight },
            9  => Card { suit, value, title: CardTitle::Nine },
            10 => Card { suit, value, title: CardTitle::Ten },
            11 => Card { suit, value, title: CardTitle::Jack },
            12 => Card { suit, value, title: CardTitle::Queen },
            13 => Card { suit, value, title: CardTitle::King },
            _  => Card { suit: Suit::Spades, value: 1, title: CardTitle::Ace },
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.title, self.suit)
    }
}

pub struct Table {
    pub deck: Vec<Card>,
    pub hands: Vec<Vec<Card>>,
    pub scrap: Vec<Vec<Card>>,
}

impl Table {
    pub fn new() -> Table {
        let mut deck: Vec<Card> = Vec::new();
        for x in 1..=52 {
            match x {
                1..=13  => deck.push(Card::new(x, Suit::Clubs)),
                14..=26 => deck.push(Card::new(x - 13, Suit::Diamonds)),
                27..=39 => deck.push(Card::new(x - 26, Suit::Hearts)),
                _       => deck.push(Card::new(x - 39, Suit::Spades)),
            }
        }
        Table {
            deck,
            hands: Vec::new(),
            scrap: Vec::new(),
        }
    }

    // pub fn deck_to_string(&self) -> String {
    //     let mut string = String::new();
    //     for x in &self.deck {
    //         string = format!("{}{}", string, x);
    //     }
    //     string
    // }

    pub fn hand_to_string(&self, hand: &Vec<Card>) -> String {
        let mut string = String::new();
        for x in hand {
            string = format!("{}{}", string, x);
        }
        string
    }

    pub fn shuffle_deck(&mut self) {
        &self.deck.shuffle(&mut thread_rng());
    }

    pub fn deal(&mut self, hands: u8, card_count: u8) {
        for _ in 1..=hands {
            &self.hands.push(Vec::new());
        }
        for _ in 1..=card_count {
            for x in &mut self.hands {
                let card = &self.deck.pop();
                match card {
                    None       => return,
                    Some(card) => x.push(card.clone()),
                }
            }
        }
    }
}
