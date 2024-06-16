use ratatui::layout::Size;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Widget};

use crate::layout::{ANSIKeyboardLayout, ISOKeyboardLayout, KeyboardLayout};

pub struct Keyboard {
    layout: KeyboardLayout,
    size: KeyboardSize,
}

#[derive(Default, Clone, Copy)]
pub enum KeyboardSize {
    #[default]
    Small,
    Large,
}

struct KeySizes {
    pub u1: Size,
    pub u1_25: Size,
    pub u1_5: Size,
    pub u1_75: Size,
    pub u2_25: Size,

    pub ansi_enter: Size,
}

struct Origin {
    pub x: u16,
    pub y: u16,
}

impl From<KeyboardSize> for KeySizes {
    fn from(value: KeyboardSize) -> Self {
        match value {
            KeyboardSize::Small => KeySizes {
                u1: Size::new(5, 3),
                u1_25: Size::new(6, 3),
                u1_5: Size::new(7, 3),
                u1_75: Size::new(9, 3),
                u2_25: Size::new(11, 3),

                ansi_enter: Size::new(10, 3),
            },
            KeyboardSize::Large => KeySizes {
                u1: Size::new(9, 5),
                u1_25: Size::new(11, 5),
                u1_5: Size::new(13, 5),
                u1_75: Size::new(16, 5),
                u2_25: Size::new(19, 5),

                ansi_enter: Size::new(19, 5),
            },
        }
    }
}

impl Keyboard {
    pub fn new(layout: KeyboardLayout, size: KeyboardSize) -> Self {
        Self { layout, size }
    }

    fn render_iso(layout: ISOKeyboardLayout, sizes: KeySizes, area: Rect, buf: &mut Buffer) {
        let mut origin = Origin {
            x: area.x,
            y: area.y,
        };

        // Render first row
        Keyboard::render_key(None, sizes.u1_5, &mut origin, buf);
        for sym in layout.row0 {
            Keyboard::render_key(Some(sym), sizes.u1, &mut origin, buf);
        }
        Keyboard::next_row(&mut origin, &area, &sizes);

        // Render second row
        Keyboard::render_key(None, sizes.u1_75, &mut origin, buf);
        for sym in layout.row1 {
            Keyboard::render_key(Some(sym), sizes.u1, &mut origin, buf);
        }
        Keyboard::next_row(&mut origin, &area, &sizes);

        // Render third row
        Keyboard::render_key(None, sizes.u1_25, &mut origin, buf);
        for sym in layout.row2 {
            Keyboard::render_key(Some(sym), sizes.u1, &mut origin, buf);
        }
        Keyboard::render_key(None, sizes.u1_75, &mut origin, buf);
    }

    fn render_ansi(layout: ANSIKeyboardLayout, sizes: KeySizes, area: Rect, buf: &mut Buffer) {
        let mut origin = Origin {
            x: area.x,
            y: area.y,
        };

        // Render first row
        Keyboard::render_key(None, sizes.u1_5, &mut origin, buf);
        let (last, rest) = layout.row0.split_last().expect("cannot be empty");
        for sym in rest {
            Keyboard::render_key(Some(sym), sizes.u1, &mut origin, buf);
        }
        Keyboard::render_key(Some(last), sizes.u1_5, &mut origin, buf);
        Keyboard::next_row(&mut origin, &area, &sizes);

        // Render second row
        Keyboard::render_key(None, sizes.u1_75, &mut origin, buf);
        for sym in layout.row1 {
            Keyboard::render_key(Some(sym), sizes.u1, &mut origin, buf);
        }
        Keyboard::render_key(None, sizes.ansi_enter, &mut origin, buf);
        Keyboard::next_row(&mut origin, &area, &sizes);

        // Render third row
        Keyboard::render_key(None, sizes.u2_25, &mut origin, buf);
        for sym in layout.row2 {
            Keyboard::render_key(Some(sym), sizes.u1, &mut origin, buf);
        }
        Keyboard::render_key(None, sizes.u1_75, &mut origin, buf);
    }

    fn render_key(symbol: Option<&str>, size: Size, origin: &mut Origin, buf: &mut Buffer) {
        let area = Rect::new(origin.x, origin.y, size.width, size.height);
        Block::bordered().render(area, buf);

        if let Some(symbol) = symbol {
            let text_x = origin.x + ((size.width - 2 /* Border */ - (symbol.len() as u16)) / 2) + 1 /* Border */;
            let text_y = origin.y + (size.height - 1/* Line Height */) / 2;

            Text::raw(symbol).render(Rect::new(text_x, text_y, symbol.len() as u16, 1), buf);
        }

        origin.x += size.width;
    }

    fn next_row(origin: &mut Origin, area: &Rect, sizes: &KeySizes) {
        origin.x = area.x;
        origin.y += sizes.u1.height;
    }

    fn get_size(key_sizes: &KeySizes, layout: &KeyboardLayout) -> Size {
        let width = match layout {
            // ISO -> Last row is the longest
            KeyboardLayout::ISO(_) => {
                key_sizes.u1_25.width + (11 * key_sizes.u1.width) + key_sizes.u1_75.width
            }

            // ANSI -> First row is the longest
            KeyboardLayout::ANSI(_) => {
                key_sizes.u1_5.width + (12 * key_sizes.u1.width) + key_sizes.u1_5.width
            }
        };

        let height = 3 * key_sizes.u1.height;

        Size::new(width, height)
    }
}

impl Widget for Keyboard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let key_sizes = self.size.into();

        let board_size = Keyboard::get_size(&key_sizes, &self.layout);

        if board_size.width > area.width || board_size.height > area.height {
            return;
        }

        let padding_x = (area.width - board_size.width) / 2;
        let padding_y = (area.height - board_size.height) / 2;
        let area = Rect::new(
            area.x + padding_x,
            area.y + padding_y,
            board_size.width,
            board_size.height,
        );

        match self.layout {
            KeyboardLayout::ISO(layout) => Keyboard::render_iso(layout, key_sizes, area, buf),
            KeyboardLayout::ANSI(layout) => Keyboard::render_ansi(layout, key_sizes, area, buf),
        }
    }
}
