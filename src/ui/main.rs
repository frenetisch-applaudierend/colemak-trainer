use ratatui::prelude::*;
use ratatui::widgets::Widget;

use crate::layout::KeyboardLayout;

use super::{Keyboard, KeyboardSize};

pub struct Main {
    pub source_layout: KeyboardLayout,
    pub target_layout: KeyboardLayout,
}

impl Widget for Main {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        Keyboard::new(self.target_layout, KeyboardSize::Small).render(areas[1], buf);
    }
}
