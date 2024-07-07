use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    state::{AppState, WordInput, WordList},
    ui::{main::Main, EventContext, RenderContext, Screen},
};

use super::MenuScreen;

pub struct TypingScreen {
    esc_count: u8,
    word_list: WordList,
    input: WordInput,
}

impl TypingScreen {
    pub fn new(state: &mut AppState) -> Self {
        let allowed_letters = state.layouts.allowed_target_letters(state.level);
        let mut word_list = WordList::new(&allowed_letters);
        let input = WordInput::new(word_list.next_word());
        Self {
            esc_count: 0,
            word_list,
            input,
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

            KeyCode::Enter => self.try_next_word(),

            KeyCode::Backspace => {
                self.input.pop();
            }

            KeyCode::Char(c) => {
                if c == ' ' {
                    self.try_next_word()
                } else {
                    self.input.push(c);
                }
            }

            _ => {}
        }
    }
}
