use std::io::{stdout, Result};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::{KeyboardSize, Main};

mod layout;
mod ui;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut size = KeyboardSize::Small;
    loop {
        terminal.draw(|frame| {
            let main = Main {
                source_layout: layout::qwertz::iso(),
                target_layout: layout::qwertz::iso(),
            };
            frame.render_widget(main, frame.size());
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('s') {
                    size = if matches!(size, KeyboardSize::Small) {
                        KeyboardSize::Large
                    } else {
                        KeyboardSize::Small
                    };
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
