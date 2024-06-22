use std::cmp::min;

use ratatui::{layout::Size, prelude::*};

pub struct Centered<T> {
    size: Size,
    child: T,
}

impl<T> Centered<T> {
    pub fn new(size: Size, child: T) -> Self {
        Self { size, child }
    }
}

impl<T> Widget for Centered<T>
where
    T: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = min(area.width, self.size.width);
        let x = area.x + ((area.width - width) / 2);

        let height = min(area.height, self.size.height);
        let y = area.y + ((area.height - height) / 2);

        self.child.render(Rect::new(x, y, width, height), buf);
    }
}
