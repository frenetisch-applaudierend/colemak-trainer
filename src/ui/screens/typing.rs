use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    keyboard::LayoutMapper,
    state::{AppState, WordInput, WordList},
    ui::{colors::ColorPalette, main::Main, EventContext, RenderContext, Screen},
};

use super::MenuScreen;

pub struct TypingScreen {
    esc_count: u8,
    word_list: WordList,
    input: WordInput,
    mapper: LayoutMapper,
    colors: ColorPalette,
}

impl TypingScreen {
    pub fn new(state: &mut AppState) -> Self {
        let allowed_letters = state.layouts.allowed_target_letters(state.level);
        let mut word_list = WordList::new(&allowed_letters);
        let input = WordInput::new(word_list.next_word());
        let mapper = state.layouts.layout_mapper();
        let colors = ColorPalette::default();

        Self {
            esc_count: 0,
            word_list,
            input,
            mapper,
            colors,
        }
    }

    fn try_next_word(&mut self) {
        if self.input.is_correct() {
            self.input = WordInput::new(self.word_list.next_word());
        }
    }
}

impl Screen for TypingScreen {
    type AppState = AppState;

    fn render(&mut self, ctx: &mut RenderContext<'_, '_, '_, Self::AppState>) {
        ctx.render_widget(Main::new(
            "Todo todo todo",
            self.input.to_line(),
            ctx.state.layouts.target_layout(),
            &self.colors,
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

            KeyCode::Enter | KeyCode::Char(' ') => {
                self.esc_count = 0;
                self.try_next_word()
            }

            KeyCode::Backspace => {
                self.esc_count = 0;
                self.input.pop();
            }

            KeyCode::Char(c) => {
                self.esc_count = 0;
                if let Some(c) = self.mapper.map(c) {
                    self.input.push(c);
                } else {
                    eprintln!("Could not map key event {:?}", event);
                }
            }

            _ => {}
        }
    }
}
