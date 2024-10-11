use std::io::{self, Write};

use crate::game::{deal_card, Game};

pub fn run() {
    let mut game = Game::default();
    loop {
        game.new_deal();
        println!("Dealer: {}", game.dealer);
        println!("Player: {}", game.player);
        while game.player.total() < 21 {
            match prompt_for_action() {
                Action::Hit => game.player.push(deal_card()),
                Action::Stand => break,
            }
            println!("Player: {}", game.player);
        }
        while game.dealer.total() <= 16 {
            game.dealer.push(deal_card());
        }

        println!("Dealer: {}", game.dealer);
        println!("{}", game.round_result());
        if !play_again() {
            println!("Y'all come back real soon!");
            return;
        }
    }
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

#[non_exhaustive]
enum Action {
    Stand,
    Hit,
}
