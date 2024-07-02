use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{Block, Padding, Paragraph};

use crate::{
    state::AppState,
    ui::{EventContext, RenderContext, Screen},
};

use super::TypingScreen;

pub struct MenuScreen {
    esc_count: u8,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self { esc_count: 0 }
    }
}

impl Screen for MenuScreen {
    type AppState = AppState;

    fn render(&mut self, ctx: &mut RenderContext<'_, '_, '_, Self::AppState>) {
        ctx.render_widget(
            Paragraph::new("Welcome to the colemak trainer. Please press <Enter> to start.\n\nPress <Esc> twice or <Q> to quit.")
            .block(Block::bordered()
                .title("Welcome")
                .padding(Padding::uniform(10))))
    }

    fn handle_event(&mut self, ctx: &mut EventContext<'_, Self::AppState>, event: KeyEvent) {
        if !event.modifiers.is_empty() {
            return;
        }

        match event.code {
            KeyCode::Esc => {
                self.esc_count += 1;
                if self.esc_count > 1 {
                    ctx.quit();
                }
            }
            KeyCode::Enter => ctx.replace_screen(TypingScreen::new()),
            KeyCode::Char(ch) => match ch {
                'q' => ctx.quit(),
                _ => {}
            },
            _ => {}
        }
    }
}
