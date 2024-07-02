use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::Paragraph;

use crate::{
    state::AppState,
    ui::{EventContext, RenderContext, Screen},
};

use super::MenuScreen;

pub struct TypingScreen {
    esc_count: u8,
}

impl TypingScreen {
    pub fn new() -> Self {
        Self { esc_count: 0 }
    }
}

impl Screen for TypingScreen {
    type AppState = AppState;

    fn render(&mut self, ctx: &mut RenderContext<'_, '_, '_, Self::AppState>) {
        ctx.render_widget(Paragraph::new("Todo"));
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

            _ => {}
        }
    }
}
