use std::io;

use hit::Tui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = Tui::default().run(&mut terminal);
    ratatui::restore();
    println!("Y'all come back real soon!");
    app_result
}
