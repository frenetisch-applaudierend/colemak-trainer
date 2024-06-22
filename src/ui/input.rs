use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Padding, Paragraph},
};

pub struct Input<'a> {
    text: &'a str,
}

impl<'a> Input<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.text)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .padding(Padding::uniform(1))
                    .title("Let's get writing!"),
            )
            .render(area, buf);
    }
}
