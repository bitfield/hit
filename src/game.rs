use std::fmt::Display;

use crate::cards::{deal, Hand};

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
        self.player.push(deal());
        self.player.push(deal());
        self.dealer.push(deal());
        self.dealer.push(deal());
        if self.player.total() >= 21 {
            self.stand();
        }
    }

    pub fn hit(&mut self) {
        self.player.push(deal());
        match self.player.total() {
            21 => self.stand(),
            22.. => self.hand_done = true,
            _ => {}
        }
    }

    pub fn stand(&mut self) {
        while self.dealer.total() <= 16 {
            self.dealer.push(deal());
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
