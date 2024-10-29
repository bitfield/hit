use std::io;

use crate::game::{Game, Phase};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::{block::Title, Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

pub struct Tui {
    phase: Phase,
    message: Line<'static>,
    game: Game,
}

impl Default for Tui {
    fn default() -> Self {
        Self {
            phase: Phase::Starting,
            message: Line::from(""),
            game: Game::default(),
        }
    }
}

impl Tui {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs the game in TUI mode.
    ///
    /// # Errors
    ///
    /// Returns any errors from Ratatui.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            match self.phase {
                Phase::Quitting => return Ok(()),
                Phase::Starting => {
                    self.phase = Phase::Playing;
                    self.game.new_deal();
                    if self.game.hand_done {
                        continue;
                    }
                    self.message = Line::from(vec![
                        "<H>".yellow().bold(),
                        "it, ".into(),
                        "<S>".yellow().bold(),
                        "tand, or ".into(),
                        "<Q>".yellow().bold(),
                        " to quit".into(),
                    ]);
                }
                Phase::Playing => {
                    if self.game.hand_done {
                        let result = self.game.round_result();
                        self.message = Line::from(vec![
                            result.to_string().into(),
                            " Press any key to continue, or ".into(),
                            "<Q>".yellow().bold(),
                            " to quit".into(),
                        ]);
                    }
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
            KeyCode::Char('q') => self.phase = Phase::Quitting,
            _ if self.game.hand_done => self.phase = Phase::Starting,
            KeyCode::Char('h') => self.game.hit(),
            KeyCode::Char('s') => self.game.stand(),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &Tui {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(" Blackjack ".bold());
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
