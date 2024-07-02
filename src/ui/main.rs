use ratatui::widgets::Widget;
use ratatui::{layout::Size, prelude::*};

use crate::layout::KeyboardLayout;

use super::{Centered, Input, Keyboard, KeyboardSize};

pub struct Main<'a> {
    target_layout: &'a KeyboardLayout,
}

impl<'a> Main<'a> {
    pub fn new(target_layout: &'a KeyboardLayout) -> Self {
        Self { target_layout }
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

        Centered::new(Size::new(80, 5), Input::new("Hello, World!")).render(areas[0], buf);
        Centered::new(Size::new(40, 5), Input::new("Foobar")).render(areas[1], buf);
        Keyboard::new(self.target_layout, KeyboardSize::Small).render(areas[2], buf);
    }
}
