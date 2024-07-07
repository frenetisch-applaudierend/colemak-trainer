use ratatui::{prelude::*, widgets::Widget};

pub struct AnyWidget(Box<dyn FnOnce(Rect, &mut Buffer) -> ()>);

impl AnyWidget {
    pub fn from(widget: impl Widget + 'static) -> Self {
        let renderer = move |area: Rect, buf: &mut Buffer| widget.render(area, buf);
        Self(Box::new(renderer))
    }
}

impl Widget for AnyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.0(area, buf)
    }
}
