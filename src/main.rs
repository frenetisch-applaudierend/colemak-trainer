use std::io::{stdout, Result};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use layout::LayoutMapper;
use ratatui::{backend::CrosstermBackend, Terminal};
use state::AppState;
use ui::{screens::MenuScreen, App};

mod layout;
mod state;
mod ui;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let source_layout = layout::qwertz::iso();
    let target_layout = layout::qwertz::iso();
    let layout_mapper = LayoutMapper::from(&source_layout, &target_layout);

    let state = AppState {
        target_layout,
        layout_mapper,
    };

    let mut app = App::new(state, MenuScreen::new());

    app.main_loop(&mut terminal)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
