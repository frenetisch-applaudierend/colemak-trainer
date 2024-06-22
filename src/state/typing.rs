use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{AppState, EventHandler, NextStep};

pub struct TypingState {
    buffer: String,
}

impl TypingState {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn current_text(&self) -> &str {
        &self.buffer
    }
}

impl EventHandler for TypingState {
    fn handle_event(&mut self, event: KeyEvent) -> super::NextStep {
        if !event.modifiers.difference(KeyModifiers::SHIFT).is_empty() {
            return NextStep::Continue;
        }

        match event.code {
            KeyCode::Esc => NextStep::NewState(AppState::menu()),
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.buffer.clear();
                NextStep::Continue
            }
            KeyCode::Backspace => {
                self.buffer.pop();
                NextStep::Continue
            }
            KeyCode::Char(ch) => {
                self.buffer.push(ch);
                NextStep::Continue
            }

            _ => NextStep::Continue,
        }
    }
}
