use std::io::{self, Write};

use crate::game::Game;

pub fn run() {
    let mut game = Game::default();
    'game: loop {
        game.new_deal();
        println!("Cash: {}", game.money);
        println!("Dealer: {}", game.dealer);
        println!("Player: {}", game.player);
        while game.player.total().value < 21 {
            match prompt_for_action() {
                Action::Hit => game.player.deal(),
                Action::Stand => break,
                Action::Quit => break 'game,
            }
            println!("Player: {}", game.player);
        }
        while game.dealer.total().value <= 16 {
            game.dealer.deal();
        }
        println!("Dealer: {}", game.dealer);
        println!("{}", game.round_result());
        game.update_money();
    }
    println!("Y'all come back real soon!");
}

fn prompt_for_action() -> Action {
    loop {
        match get_player_input("(H)it, (S)tand, or (Q)uit?").as_str() {
            "s" | "S" => return Action::Stand,
            "h" | "H" => return Action::Hit,
            "q" | "Q" => return Action::Quit,
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
    Quit
}
