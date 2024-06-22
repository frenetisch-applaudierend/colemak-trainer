use crossterm::event::{KeyCode, KeyEvent};

use super::{AppState, EventHandler, NextStep};

pub struct MenuState {
    esc_count: u8,
}

impl MenuState {
    pub fn new() -> Self {
        Self { esc_count: 0 }
    }
}

impl EventHandler for MenuState {
    fn handle_event(&mut self, event: KeyEvent) -> NextStep {
        if !event.modifiers.is_empty() {
            return NextStep::Continue;
        }

        match event.code {
            KeyCode::Esc => {
                self.esc_count += 1;
                if self.esc_count > 1 {
                    NextStep::Quit
                } else {
                    NextStep::Continue
                }
            }
            KeyCode::Enter => NextStep::NewState(AppState::typing()),
            KeyCode::Char(ch) => match ch {
                'q' => NextStep::Quit,
                _ => NextStep::Continue,
            },

            _ => NextStep::Continue,
        }
    }
}
