use std::io::{stdout, Result};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use layout::KeyboardLayout;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use ui::{Keyboard, KeyboardSize};

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
            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.size());

            let keyboard = Keyboard::new(KeyboardLayout::ANSI(layout::qwerty::ansi()), size);
            frame.render_widget(keyboard, areas[0]);

            let keyboard = Keyboard::new(KeyboardLayout::ISO(layout::qwerty::iso()), size);
            frame.render_widget(keyboard, areas[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
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
