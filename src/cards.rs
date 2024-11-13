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

#[derive(Debug, Eq, PartialEq)]
pub struct Total {
    pub value: usize,
    soft: bool,
}

impl Display for Total {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            if self.soft { "soft" } else { "total" },
            self.value
        )
    }
}

#[derive(Default)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total(&self) -> Total {
        let mut total = Total {
            value: self
                .0
                .iter()
                .fold(0, |total, next| total + next.rank.value()),
            soft: false,
        };
        if self.0.iter().any(|card| card.rank == Rank::Ace) && total.value <= 11 {
            total.value += 10;
            total.soft = true;
        }
        total
    }

    pub fn deal(&mut self) {
        self.0.push(Card {
            rank: random(),
            suit: random(),
        });
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|card| write!(f, "{card}"))?;
        write!(f, "({})", self.total())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_values_aces_as_11_unless_total_would_exceed_21() {
        let mut hand = Hand::new();
        add(&mut hand, Rank::Ace);
        add(&mut hand, Rank::Five);
        assert_eq!(
            hand.total(),
            Total {
                value: 16,
                soft: true
            }
        );
        add(&mut hand, Rank::Ten);
        assert_eq!(
            hand.total(),
            Total {
                value: 16,
                soft: false
            }
        );
    }

    fn add(hand: &mut Hand, rank: Rank) {
        hand.0.push(Card {
            suit: Suit::Spades,
            rank,
        });
    }
}
