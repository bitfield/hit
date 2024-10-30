use std::io;

use hit::Tui;

fn main() -> io::Result<()> {
    let result = if std::env::args().any(|arg| arg == "--cli") {
        hit::run_cli();
        Ok(())
    } else {
        let mut terminal = ratatui::init();
        let app_result = Tui::default().run(&mut terminal);
        ratatui::restore();
        app_result
    };
    println!("Y'all come back real soon!");
    result
}
