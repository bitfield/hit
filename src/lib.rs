use rand::Rng;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::{block::Title, Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(PartialEq)]
enum Phase {
    Start,
    Playing,
    Result,
    Quit,
}

pub struct Game {
    player: Hand,
    dealer: Hand,
    hand_done: bool,
}

pub struct Tui {
    phase: Phase,
    message: Line<'static>,
    game: Game,
}

impl Default for Tui {
    fn default() -> Self {
        Self {
            phase: Phase::Start,
            message: Line::from(""),
            game: Game::default(),
        }
    }
} 

impl Default for Game {
    fn default() -> Self {
        Self {
            player: Hand::default(),
            dealer: Hand::default(),
            hand_done: false,
        }
    }
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
   
    fn new_deal(&mut self) {
        self.hand_done = false;
        self.player = Hand::new();
        self.dealer = Hand::new();
        self.player.push(deal());
        self.player.push(deal());
        self.dealer.push(deal());
        self.dealer.push(deal());
    }

    fn hit(&mut self) {
        self.player.push(deal());
        match self.player.total() {
            21 => self.stand(),
            22.. => self.hand_done = true,
            _ => {},
        }
    }

    fn stand(&mut self) {
        while self.dealer.total() <= 16 {
            self.dealer.push(deal());
        }
        self.hand_done = true;
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

impl Tui {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
        match self.phase {
            Phase::Quit => return Ok(()),
            Phase::Start => {
                self.game.new_deal();
                self.message = Line::from(vec![
                    "<H>".yellow().bold(),
                    "it, ".into(),
                    "<S>".yellow().bold(),
                    "tand, or ".into(),
                    "<Q>".yellow().bold(),
                    " to quit".into(),
                ]);
                self.phase = Phase::Playing;
            }
            Phase::Playing => {
                if self.game.hand_done {

                self.message = Line::from("hand done");
                    self.phase = Phase::Result;
                }
            }
            Phase::Result => {
                let result = self.game.round_result();
                self.message = Line::from(vec![
                    result.to_string().into(),
                    " Press any key to continue, or ".into(),
                    "<Q>".yellow().bold(),
                    " to quit".into(),
                ]);
            }
        };
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.phase = Phase::Quit,
            KeyCode::Char('r') => {},
            _ if self.phase == Phase::Result => self.phase = Phase::Start,
            KeyCode::Char('h') => self.game.hit(),
            KeyCode::Char('s') => self.game.stand(),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn cli(&mut self) {
        loop {
            self.game.new_deal();
            println!("Dealer: {}", self.game.dealer);
            println!("Player: {}", self.game.player);
            while self.game.player.total() < 21 {
                match prompt_for_action() {
                    Action::Hit => self.game.player.push(deal()),
                    Action::Stand => break,
                }
                println!("Player: {}", self.game.player);
            }
            while self.game.dealer.total() <= 16 {
                self.game.dealer.push(deal());
            }

            println!("Dealer: {}", self.game.dealer);
            println!("{}", self.game.round_result());
            if !play_again() {
                println!("Y'all come back real soon!");
                return;
            }
        }
    }

 }

impl Widget for &Tui {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(" Blackjack! ".bold());
        let block = Block::new().title(title.alignment(Alignment::Center));
        let window = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [dealer_hand_area, player_hand_area, message_area] = window.areas(block.inner(area));
        Paragraph::new(self.game.dealer.to_string())
            .block(Block::bordered().title(Title::from("Dealer").alignment(Alignment::Center)))
            .alignment(Alignment::Center)
            .render(dealer_hand_area, buf);
        Paragraph::new(self.game.player.to_string())
            .block(Block::bordered().title(Title::from("Player").alignment(Alignment::Center)))
            .alignment(Alignment::Center)
            .render(player_hand_area, buf);
        Paragraph::new(self.message.clone())
            .alignment(Alignment::Center)
            .render(message_area, buf);
        block.render(area, buf);
    }
}

enum RoundResult {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hint() {
    }
}
