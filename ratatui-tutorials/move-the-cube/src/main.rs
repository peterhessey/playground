use std::{error::Error, io};
mod app;
mod ui;
use crate::app::App;
use crate::ui::ui;
use ratatui::{
    DefaultTerminal, Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    let terminal = ratatui::init();
    let mut app = App::new();
    let _ = run_app(terminal, &mut app);
    ratatui::restore();
    Ok(())
}
fn run_app(mut terminal: DefaultTerminal, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            // only process key presses
            if key.kind != event::KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Left => app.move_left(),
                KeyCode::Right => app.move_right(),
                _ => continue,
            }
        }
    }
}
