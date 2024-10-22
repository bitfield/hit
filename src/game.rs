use std::{
    collections::{hash_set::IntoIter, HashSet},
    fmt::Display,
};

#[derive(Default)]
pub struct Game {
    pub player: Hand,
    pub dealer: Hand,
    pub hand_done: bool,
    deck: Deck,
}

impl Game {
    pub fn deal(&mut self) -> Card {
        self.deck.deal()
    }

    pub fn new_deal(&mut self) {
        self.hand_done = false;
        self.player = Hand::new();
        self.dealer = Hand::new();
        self.player.push(self.deck.deal());
        self.player.push(self.deck.deal());
        self.dealer.push(self.deck.deal());
        self.dealer.push(self.deck.deal());
    }

    pub fn hit(&mut self) {
        self.player.push(self.deck.deal());
        match self.player.total() {
            21 => self.stand(),
            22.. => self.hand_done = true,
            _ => {}
        }
    }

    pub fn stand(&mut self) {
        while self.dealer.total() <= 16 {
            self.dealer.push(self.deck.deal());
        }
        self.hand_done = true;
    }

    pub fn round_result(&self) -> RoundResult {
        let p = self.player.total();
        let d = self.dealer.total();
        if p > 21 {
            RoundResult::PlayerBust
        } else if d > 21 {
            RoundResult::DealerBust
        } else if p > d {
            RoundResult::PlayerWins
        } else if d > p {
            RoundResult::DealerWins
        } else {
            RoundResult::Tie
        }
    }
}

#[derive(PartialEq)]
pub enum Phase {
    Starting,
    Playing,
    Quitting,
}

pub enum RoundResult {
    PlayerBust,
    DealerWins,
    DealerBust,
    PlayerWins,
    Tie,
}

impl Display for RoundResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_str = match self {
            RoundResult::PlayerBust => "Bust!",
            RoundResult::DealerBust => "Dealer bust, you win!",
            RoundResult::PlayerWins => "You win!",
            RoundResult::DealerWins => "Dealer wins!",
            RoundResult::Tie => "It's a tie!",
        };
        write!(f, "{result_str}")
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} ", self.suit, self.rank)
    }
}

#[derive(Default)]
pub struct Hand(Vec<Card>);

impl Hand {
    fn new() -> Self {
        Self::default()
    }

    pub fn total(&self) -> usize {
        self.0.iter().fold(0, |total, next| total + next.rank)
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

#[derive(Clone, Eq, Hash, PartialEq)]
enum Suit {
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

struct Deck(IntoIter<Card>);

impl Deck {
    pub fn deal(&mut self) -> Card {
        self.0.next().expect("ran out of cards")
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = HashSet::with_capacity(52);
        for rank in 1..=13 {
            for suit in &[Suit::Spades, Suit::Clubs, Suit::Diamonds, Suit::Hearts] {
                cards.insert(Card {
                    suit: suit.clone(),
                    rank,
                });
            }
        }
        Deck(cards.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deck_contains_52_cards() {
        let deck = Deck::default();
        assert_eq!(deck.len(), 52);
    }
}
