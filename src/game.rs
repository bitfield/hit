use std::fmt::Display;

use crate::cards::Hand;

pub struct Game {
    pub money: usize,
    pub bet: usize,
    pub player: Hand,
    pub dealer: Hand,
    pub hand_done: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            money: 100,
            bet: 5,
            player: Hand::default(),
            dealer: Hand::default(),
            hand_done: false,
        }
    }
}

impl Game {
    pub fn new_deal(&mut self) {
        self.hand_done = false;
        self.money = self.money.checked_sub(self.bet).expect("You're broke!");
        self.player = Hand::new();
        self.dealer = Hand::new();
        self.player.deal();
        self.player.deal();
        self.dealer.deal();
        self.dealer.deal();
        if self.player.total().value >= 21 {
            self.stand();
        }
    }

    pub fn hit(&mut self) {
        self.player.deal();
        match self.player.total().value {
            21 => self.stand(),
            22.. => self.hand_done = true,
            _ => {}
        }
    }

    pub fn stand(&mut self) {
        while self.dealer.total().value <= 16 {
            self.dealer.deal();
        }
        self.hand_done = true;
    }

    pub fn round_result(&self) -> RoundResult {
        let p = self.player.total().value;
        let d = self.dealer.total().value;
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

    pub fn update_money(&mut self) {
        match self.round_result() {
            RoundResult::Tie => self.money += self.bet,
            RoundResult::PlayerWins | RoundResult::DealerBust => {
                if self.player.total().value == 21 {
                    self.money += self.bet * 3;
                } else {
                    self.money += self.bet * 2;
                }
            }
            _ => {}
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
