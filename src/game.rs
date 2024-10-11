use std::fmt::Display;

use rand::Rng;

#[derive(Default)]
pub struct Game {
    pub player: Hand,
    pub dealer: Hand,
    pub hand_done: bool,
}

impl Game {
    pub fn new_deal(&mut self) {
        self.hand_done = false;
        self.player = Hand::new();
        self.dealer = Hand::new();
        self.player.push(deal_card());
        self.player.push(deal_card());
        self.dealer.push(deal_card());
        self.dealer.push(deal_card());
    }

    pub fn hit(&mut self) {
        self.player.push(deal_card());
        match self.player.total() {
            21 => self.stand(),
            22.. => self.hand_done = true,
            _ => {}
        }
    }

    pub fn stand(&mut self) {
        while self.dealer.total() <= 16 {
            self.dealer.push(deal_card());
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

#[derive(Copy, Clone)]
pub struct Card(usize);

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.0)
    }
}

pub fn deal_card() -> Card {
    Card(rand::thread_rng().gen_range(1..=10))
}

#[derive(Default)]
pub struct Hand(Vec<Card>);

impl Hand {
    fn new() -> Self {
        Self::default()
    }

    pub fn total(&self) -> usize {
        self.0.iter().fold(0, |total, next| total + next.0)
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

