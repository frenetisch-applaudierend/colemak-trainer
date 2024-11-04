use ratatui::widgets::Widget;
use ratatui::{layout::Size, prelude::*};

use crate::layout::AnyKeyboardLayout;

use super::{Centered, Input, Keyboard};

pub struct Main<'a> {
    word_list: &'a str,
    input: Line<'a>,
    target_layout: AnyKeyboardLayout<'a>,
}

impl<'a> Main<'a> {
    pub fn new(word_list: &'a str, input: Line<'a>, target_layout: AnyKeyboardLayout<'a>) -> Self {
        Self {
            word_list,
            input,
            target_layout,
        }
    }
}

impl Widget for Main<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ])
            .split(area);

        Centered::new(Size::new(80, 5), Input::new(Line::raw(self.word_list)))
            .render(areas[0], buf);
        Centered::new(Size::new(40, 5), Input::new(self.input)).render(areas[1], buf);
        Keyboard::new(self.target_layout).render(areas[2], buf);
    }
}
