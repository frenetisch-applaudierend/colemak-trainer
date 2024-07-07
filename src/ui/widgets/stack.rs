use ratatui::{layout::Constraint, prelude::*, widgets::Widget};

use super::AnyWidget;

pub struct VStack {
    items: Vec<StackItem>,
}

impl VStack {
    pub fn new(items: impl Into<Vec<StackItem>>) -> Self {
        Self {
            items: items.into(),
        }
    }
}

impl Widget for VStack {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::vertical(self.items.iter().map(|i| i.constraint)).split(area);
        let widgets = self.items.into_iter().map(|i| i.widget);

        for (area, widget) in areas.into_iter().zip(widgets) {
            widget.render(*area, buf);
        }
    }
}

pub struct StackItem {
    pub widget: AnyWidget,
    pub constraint: Constraint,
}

impl StackItem {
    pub fn new(widget: impl Widget + 'static, constraint: Constraint) -> Self {
        Self {
            widget: AnyWidget::from(widget),
            constraint,
        }
    }
}
