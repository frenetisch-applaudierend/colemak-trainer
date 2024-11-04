use ratatui::prelude::*;
use symbols::line;

pub struct IsoEnter {
    pub width: u16,
    pub line_height: u16,
    pub inset: u16,
}

impl Widget for IsoEnter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.width < 2
            || self.line_height < 2
            || self.width > area.width
            || self.line_height * 2 > area.height
        {
            return;
        }

        // Top border
        buf.get_mut(area.left(), area.top())
            .set_symbol(line::ROUNDED_TOP_LEFT);
        for i in 1..(self.width - 1) {
            buf.get_mut(area.left() + i, area.top())
                .set_symbol(line::HORIZONTAL);
        }
        buf.get_mut(area.left() + self.width - 1, area.top())
            .set_symbol(line::ROUNDED_TOP_RIGHT);

        // Top middle part
        for l in 1..(self.line_height - 1) {
            buf.get_mut(area.left(), area.top() + l)
                .set_symbol(line::VERTICAL);
            for i in 1..(self.width - 1) {
                buf.get_mut(area.left() + i, area.top() + l).set_symbol(" ");
            }
            buf.get_mut(area.left() + self.width - 1, area.top() + l)
                .set_symbol(line::VERTICAL);
        }

        // Inset part
        buf.get_mut(area.left(), area.top() + self.line_height - 1)
            .set_symbol(line::ROUNDED_BOTTOM_LEFT);
        for i in 1..self.inset {
            buf.get_mut(area.left() + i, area.top() + self.line_height - 1)
                .set_symbol(line::HORIZONTAL);
        }
        buf.get_mut(area.left() + self.inset, area.top() + self.line_height - 1)
            .set_symbol(line::ROUNDED_TOP_RIGHT);
        buf.get_mut(
            area.left() + self.width - 1,
            area.top() + self.line_height - 1,
        )
        .set_symbol(line::VERTICAL);

        // Bottom middle part
        for l in 0..(self.line_height - 1) {
            buf.get_mut(area.left() + self.inset, area.top() + self.line_height + l)
                .set_symbol(line::VERTICAL);
            buf.get_mut(
                area.left() + self.width - 1,
                area.top() + self.line_height + l,
            )
            .set_symbol(line::VERTICAL);
        }

        // Bottom border
        buf.get_mut(
            area.left() + self.inset,
            area.top() + 2 * self.line_height - 1,
        )
        .set_symbol(line::ROUNDED_BOTTOM_LEFT);
        for i in (self.inset + 1)..(self.width - 1) {
            buf.get_mut(area.left() + i, area.top() + 2 * self.line_height - 1)
                .set_symbol(line::HORIZONTAL);
        }
        buf.get_mut(
            area.left() + self.width - 1,
            area.top() + 2 * self.line_height - 1,
        )
        .set_symbol(line::ROUNDED_BOTTOM_RIGHT);
    }
}
