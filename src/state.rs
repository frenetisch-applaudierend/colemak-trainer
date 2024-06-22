use crossterm::event::KeyEvent;

pub use self::{menu::MenuState, typing::TypingState};

mod menu;
mod typing;

pub trait EventHandler {
    fn handle_event(&mut self, event: KeyEvent) -> NextStep;
}

pub enum NextStep {
    Continue,
    NewState(AppState),
    Quit,
}

pub enum AppState {
    Menu(MenuState),
    Typing(TypingState),
}

impl AppState {
    pub fn menu() -> Self {
        Self::Menu(MenuState::new())
    }

    pub fn typing() -> Self {
        Self::Typing(TypingState::new())
    }
}

impl EventHandler for AppState {
    fn handle_event(&mut self, event: KeyEvent) -> NextStep {
        match self {
            AppState::Menu(s) => s.handle_event(event),
            AppState::Typing(s) => s.handle_event(event),
        }
    }
}
