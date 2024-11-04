use ratatui::layout::Size;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Widget};

use crate::layout::{AnsiKeyboardLayout, AnyKeyboardLayout, IsoKeyboardLayout, Key};

use super::iso_enter::IsoEnter;

pub struct Keyboard<'a> {
    layout: AnyKeyboardLayout<'a>,
}

struct KeySizes {
    pub u1: Size,
    pub u1_25: Size,
    pub u1_5: Size,
    pub u1_75: Size,
    pub u2: Size,
    pub u2_25: Size,
}

#[derive(Debug, Clone, Copy)]
struct Origin {
    pub x: u16,
    pub y: u16,
}

static KEY_HEIGHT: u16 = 3;

static SIZES: KeySizes = KeySizes {
    u1: Size::new(5, KEY_HEIGHT),
    u1_25: Size::new(6, KEY_HEIGHT),
    u1_5: Size::new(7, KEY_HEIGHT),
    u1_75: Size::new(9, KEY_HEIGHT),
    u2: Size::new(10, KEY_HEIGHT),
    u2_25: Size::new(11, KEY_HEIGHT),
};

static BOARD_SIZE: Size = Size {
    width: SIZES.u1.width * 13 + SIZES.u2.width,
    height: 5 * SIZES.u1.height,
};

impl<'a> Keyboard<'a> {
    pub fn new(layout: AnyKeyboardLayout<'a>) -> Self {
        Self { layout }
    }

    fn render_layout(layout: AnyKeyboardLayout<'a>, area: Rect, buf: &mut Buffer) {
        let mut origin = Origin {
            x: area.x,
            y: area.y,
        };

        // Render top row
        for _ in 0..13 {
            Keyboard::render_key(Key::None, SIZES.u1, &mut origin, buf);
        }
        Keyboard::render_key(Key::None, SIZES.u2, &mut origin, buf);
        Keyboard::next_row(&mut origin, &area);

        match layout {
            AnyKeyboardLayout::Iso(layout) => Keyboard::render_iso(layout, area, &mut origin, buf),
            AnyKeyboardLayout::Ansi(layout) => {
                Keyboard::render_ansi(layout, area, &mut origin, buf)
            }
        }

        // Render bottom row
        Keyboard::next_row(&mut origin, &area);
        for _ in 0..3 {
            Keyboard::render_key(Key::None, SIZES.u1_25, &mut origin, buf);
        }

        let space_size = Size::new(BOARD_SIZE.width - 7 * SIZES.u1_25.width, KEY_HEIGHT);
        Keyboard::render_key(Key::None, space_size, &mut origin, buf);

        for _ in 0..4 {
            Keyboard::render_key(Key::None, SIZES.u1_25, &mut origin, buf);
        }
    }

    fn render_iso(layout: &IsoKeyboardLayout, area: Rect, origin: &mut Origin, buf: &mut Buffer) {
        // Render first key row
        Keyboard::render_key(Key::None, SIZES.u1_5, origin, buf);
        for sym in layout.row0 {
            Keyboard::render_key(sym, SIZES.u1, origin, buf);
        }

        let iso_enter_width = Keyboard::remaining_key_size(area, *origin).width;
        let iso_enter = IsoEnter {
            width: iso_enter_width,
            line_height: KEY_HEIGHT,
            inset: 2,
        };
        iso_enter.render(
            Rect::new(origin.x, origin.y, iso_enter_width, KEY_HEIGHT * 2),
            buf,
        );

        Keyboard::next_row(origin, &area);

        // Render second key row
        Keyboard::render_key(Key::None, SIZES.u1_75, origin, buf);
        for sym in layout.row1 {
            Keyboard::render_key(sym, SIZES.u1, origin, buf);
        }
        Keyboard::next_row(origin, &area);

        // Render third key row
        Keyboard::render_key(Key::None, SIZES.u1_25, origin, buf);
        for sym in layout.row2 {
            Keyboard::render_key(sym, SIZES.u1, origin, buf);
        }
        let rshift_size = Keyboard::remaining_key_size(area, *origin);
        Keyboard::render_key(Key::None, rshift_size, origin, buf);
    }

    fn render_ansi(layout: &AnsiKeyboardLayout, area: Rect, origin: &mut Origin, buf: &mut Buffer) {
        // Render first key row
        Keyboard::render_key(Key::None, SIZES.u1_5, origin, buf);
        let (last, rest) = layout.row0.split_last().expect("cannot be empty");
        for sym in rest {
            Keyboard::render_key(*sym, SIZES.u1, origin, buf);
        }
        let last_key_size = Keyboard::remaining_key_size(area, *origin);
        Keyboard::render_key(*last, last_key_size, origin, buf);
        Keyboard::next_row(origin, &area);

        // Render second key row
        Keyboard::render_key(Key::None, SIZES.u1_75, origin, buf);
        for sym in layout.row1 {
            Keyboard::render_key(sym, SIZES.u1, origin, buf);
        }
        let enter_size = Keyboard::remaining_key_size(area, *origin);
        Keyboard::render_key(Key::None, enter_size, origin, buf);
        Keyboard::next_row(origin, &area);

        // Render third key row
        Keyboard::render_key(Key::None, SIZES.u2_25, origin, buf);
        for sym in layout.row2 {
            Keyboard::render_key(sym, SIZES.u1, origin, buf);
        }
        let rshift_size = Keyboard::remaining_key_size(area, *origin);
        Keyboard::render_key(Key::None, rshift_size, origin, buf);
    }

    fn render_key(symbol: Key, size: Size, origin: &mut Origin, buf: &mut Buffer) {
        let area = Rect::new(origin.x, origin.y, size.width, size.height);
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        if let Key::Char(symbol) = symbol {
            let symbol = &symbol.to_string();
            let text_x = origin.x + ((size.width - 2 /* Border */ - (symbol.len() as u16)) / 2) + 1 /* Border */;
            let text_y = origin.y + (size.height - 1/* Line Height */) / 2;

            Text::raw(symbol).render(Rect::new(text_x, text_y, symbol.len() as u16, 1), buf);
        }

        origin.x += size.width;
    }

    fn next_row(origin: &mut Origin, area: &Rect) {
        origin.x = area.x;
        origin.y += SIZES.u1.height;
    }

    fn remaining_key_size(area: Rect, origin: Origin) -> Size {
        let width = BOARD_SIZE.width - (origin.x - area.left());
        Size::new(width, KEY_HEIGHT)
    }
}

impl Widget for Keyboard<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if BOARD_SIZE.width > area.width || BOARD_SIZE.height > area.height {
            return;
        }

        let padding_x = (area.width - BOARD_SIZE.width) / 2;
        let padding_y = (area.height - BOARD_SIZE.height) / 2;
        let area = Rect::new(
            area.x + padding_x,
            area.y + padding_y,
            BOARD_SIZE.width,
            BOARD_SIZE.height,
        );

        Keyboard::render_layout(self.layout, area, buf);
    }
}
