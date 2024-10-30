use rand::{distributions::Standard, prelude::*, random, Rng};

use std::fmt::Display;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} ",
            match self.rank {
                Rank::Ace => "A".into(),
                Rank::Two
                | Rank::Three
                | Rank::Four
                | Rank::Five
                | Rank::Six
                | Rank::Seven
                | Rank::Eight
                | Rank::Nine
                | Rank::Ten => self.rank.value().to_string(),
                Rank::Jack => "J".into(),
                Rank::Queen => "Q".into(),
                Rank::King => "K".into(),
            },
            self.suit
        )
    }
}

pub fn deal() -> Card {
    Card {
        rank: random(),
        suit: random(),
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Spades => "♠",
                Suit::Clubs => "♣",
                Suit::Diamonds => "♦",
                Suit::Hearts => "♥",
            }
        )
    }
}

impl Distribution<Suit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        match rng.gen_range(0..=3) {
            0 => Suit::Spades,
            1 => Suit::Clubs,
            2 => Suit::Diamonds,
            3 => Suit::Hearts,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Rank {
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

impl Rank {
    fn value(self) -> usize {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

impl Distribution<Rank> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rank {
        match rng.gen_range(1..=13) {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total(&self) -> usize {
        self.0
            .iter()
            .fold(0, |total, next| total + next.rank.value())
    }

    pub fn push(&mut self, card: Card) {
        self.0.push(card);
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|card| write!(f, "{card}"))?;
        write!(f, "(total {})", self.total())
    }
}
