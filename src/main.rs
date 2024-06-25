use std::io::{stdout, Result};

use crossterm::{
    event::{self, Event, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use layout::{KeyboardLayout, LayoutMapper};
use ratatui::{backend::CrosstermBackend, Terminal};
use state::{AppState, EventHandler, NextStep};
use ui::Main;

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
    let mapper = LayoutMapper::from(&source_layout, &target_layout);

    let mut state = AppState::menu();

    loop {
        terminal.draw(|frame| match &state {
            AppState::Menu(_) => {}
            AppState::Typing(state) => {
                let main = Main::new(&source_layout, &target_layout, &state);
                frame.render_widget(main, frame.size());
            }
        })?;

        if matches!(handle_events(&mut state)?, Action::Quit) {
            break;
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn handle_events(state: &mut AppState) -> Result<Action> {
    if !event::poll(std::time::Duration::from_millis(16))? {
        return Ok(Action::Continue);
    }

    let Event::Key(key) = event::read()? else {
        return Ok(Action::Continue);
    };

    if key.kind != KeyEventKind::Press {
        return Ok(Action::Continue);
    }

    Ok(match state.handle_event(key) {
        NextStep::Continue => Action::Continue,
        NextStep::NewState(next) => {
            *state = next;
            Action::Continue
        }
        NextStep::Quit => Action::Quit,
    })
}

enum Action {
    Continue,
    Quit,
}
