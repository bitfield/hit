use std::{
    fmt::Display,
    io::{self, Write},
};

use rand::Rng;

fn main() {
    let mut playing = true;
    while playing {
        let mut player = Hand::new();
        let mut dealer = Hand::new();
        player.push(deal());
        player.push(deal());
        dealer.push(deal());
        dealer.push(deal());
        println!("Dealer: {dealer}");
        println!("Player: {player}");
        while player.total() < 21 {
            match prompt_for_action() {
                Action::Hit => player.push(deal()),
                Action::Stand => break,
            }
            println!("Player: {player}");
        }
        while dealer.total() <= 16 {
            dealer.push(deal());
        }
        let p = player.total();
        let d = dealer.total();
        println!("Dealer: {dealer}");
        if p > 21 {
            println!("Bust!");
        } else if d > 21 {
            println!("Dealer bust, you win!");
        } else if p > d {
            println!("You win!");
        } else if d > p {
            println!("Dealer wins!");
        } else {
            println!("It's a tie!");
        }
        playing = prompt_to_play_again();
    }
    println!("Y'all come back real soon!");
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

struct Hand(Vec<Card>);

impl Hand {
    fn new() -> Self {
        Self(Vec::new())
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

fn prompt_to_play_again() -> bool {
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
