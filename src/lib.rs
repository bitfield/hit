use rand::Rng;
use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(Default)]
pub struct Game {
    player: Hand,
    dealer: Hand,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cli(&mut self) {
        loop {
            self.new_deal();
            println!("Dealer: {}", self.dealer);
            println!("Player: {}", self.player);
            while self.player.total() < 21 {
                match prompt_for_action() {
                    Action::Hit => self.player.push(deal()),
                    Action::Stand => break,
                }
                println!("Player: {}", self.player);
            }
            while self.dealer.total() <= 16 {
                self.dealer.push(deal());
            }

            println!("Dealer: {}", self.dealer);
            match self.round_result() {
                RoundResult::PlayerBust => println!("Bust!"),
                RoundResult::DealerBust => println!("Dealer bust, you win!"),
                RoundResult::PlayerWins => println!("You win!"),
                RoundResult::DealerWins => println!("Dealer wins!"),
                RoundResult::Tie => println!("It's a tie!"),
            }
            if !play_again() {
                println!("Y'all come back real soon!");
                return;
            }
        }
    }

    fn new_deal(&mut self) {
        self.player = Hand::new();
        self.dealer = Hand::new();
        self.player.push(deal());
        self.player.push(deal());
        self.dealer.push(deal());
        self.dealer.push(deal());
    }

    fn round_result(&self) -> RoundResult {
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

enum RoundResult {
    PlayerBust,
    DealerWins,
    DealerBust,
    PlayerWins,
    Tie,
}

#[derive(Copy, Clone)]
struct Card(usize);

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.0)
    }
}

fn deal() -> Card {
    Card(rand::thread_rng().gen_range(1..=10))
}

#[derive(Default)]
struct Hand(Vec<Card>);

impl Hand {
    fn new() -> Self {
        Self::default()
    }

    fn total(&self) -> usize {
        self.0.iter().fold(0, |total, next| total + next.0)
    }

    fn push(&mut self, card: Card) {
        self.0.push(card);
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|card| write!(f, "{card}"))?;
        write!(f, "(total {})", self.total())
    }
}

#[non_exhaustive]
enum Action {
    Stand,
    Hit,
}

fn prompt_for_action() -> Action {
    loop {
        match get_player_input("(h)it or (s)tand?").as_str() {
            "s" => return Action::Stand,
            "h" => return Action::Hit,
            _ => println!("Sorry, I'm not sure what you want to do."),
        }
    }
}

fn play_again() -> bool {
    loop {
        match get_player_input("Play again (y/n)?").as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Sorry, I'm not sure what you want to do."),
        }
    }
}

fn get_player_input(prompt: &'static str) -> String {
    let mut choice = String::new();
    print!("{prompt} > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();
    choice.trim_end().to_string()
}
