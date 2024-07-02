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
    pub fn new() -> Self {
        Self {
            esc_count: 0,
            input: WordInput::new("hello"),
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

            KeyCode::Enter => {
                self.input = WordInput::new("foobar");
            }

            KeyCode::Backspace => {
                self.input.pop();
            }

            KeyCode::Char(c) => {
                self.input.push(c);
            }

            _ => {}
        }
    }
}
