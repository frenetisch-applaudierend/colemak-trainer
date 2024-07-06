use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::Paragraph;

use crate::{
    state::{AppState, WordInput},
    ui::{main::Main, EventContext, RenderContext, Screen},
};

use super::MenuScreen;

pub struct TypingScreen {
    esc_count: u8,
    input: WordInput,
}

impl TypingScreen {
    pub fn new(state: &mut AppState) -> Self {
        Self {
            esc_count: 0,
            input: state.next_word(),
        }
    }

    fn try_next_word(&mut self, state: &mut AppState) {
        if self.input.is_correct() {
            self.input = state.next_word();
        }
    }
}

impl Screen for TypingScreen {
    type AppState = AppState;

    fn render(&mut self, ctx: &mut RenderContext<'_, '_, '_, Self::AppState>) {
        ctx.render_widget(Main::new(
            "Todo todo todo",
            self.input.to_line(),
            &ctx.state.target_layout,
        ));
    }

    fn handle_event(&mut self, ctx: &mut EventContext<'_, Self::AppState>, event: KeyEvent) {
        if !event.modifiers.is_empty() {
            return;
        }

        match event.code {
            KeyCode::Esc => {
                self.esc_count += 1;
                if self.esc_count > 1 {
                    ctx.replace_screen(MenuScreen::new());
                }
            }

            KeyCode::Enter => self.try_next_word(&mut ctx.state),

            KeyCode::Backspace => {
                self.input.pop();
            }

            KeyCode::Char(c) => {
                if c == ' ' {
                    self.try_next_word(&mut ctx.state)
                } else {
                    self.input.push(c);
                }
            }

            _ => {}
        }
    }
}
