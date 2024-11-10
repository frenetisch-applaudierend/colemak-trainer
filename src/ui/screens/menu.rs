use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{Block, Padding, Paragraph};

use crate::{
    keyboard::{layouts, AnsiKeyboardLayout, IsoKeyboardLayout},
    state::AppState,
    ui::{EventContext, RenderContext, Screen},
};

use super::TypingScreen;

pub struct MenuScreen {
    iso_layouts: Vec<IsoKeyboardLayout>,
    ansi_layouts: Vec<AnsiKeyboardLayout>,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self {
            iso_layouts: vec![
                layouts::qwerty::iso(),
                layouts::qwertz::iso(),
                layouts::colemak_dh::iso(),
            ],
            ansi_layouts: vec![
                layouts::qwerty::ansi(),
                layouts::qwertz::ansi(),
                layouts::colemak_dh::ansi(),
            ],
        }
    }
}

impl Screen for MenuScreen {
    type AppState = AppState;

    fn render(&mut self, ctx: &mut RenderContext<'_, '_, '_, Self::AppState>) {
        ctx.render_widget(
            Paragraph::new("Welcome to the colemak trainer. Please press <Enter> to start.\n\nPress <Esc> or <Q> to quit.")
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
                ctx.quit();
            }
            KeyCode::Enter => {
                let screen = TypingScreen::new(&mut ctx.state);
                ctx.replace_screen(screen);
            }
            KeyCode::Char(ch) => match ch {
                'q' => ctx.quit(),
                _ => {}
            },
            _ => {}
        }
    }
}
