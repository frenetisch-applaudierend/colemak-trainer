use ratatui::layout::Size;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Widget};

use crate::keyboard::{self, AnsiKeyboardLayout, AnyKeyboardLayout, IsoKeyboardLayout};

use super::colors::ColorPalette;
use super::iso_enter::IsoEnter;

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

pub struct Keyboard<'a> {
    layout: AnyKeyboardLayout<'a>,
    colors: &'a ColorPalette,
    origin: Origin,
}

impl<'a> Keyboard<'a> {
    pub fn new(layout: AnyKeyboardLayout<'a>, colors: &'a ColorPalette) -> Self {
        Self {
            layout,
            colors,
            origin: Default::default(),
        }
    }

    fn render_layout(mut self, area: Rect, buf: &mut Buffer) {
        self.origin = Origin {
            x: area.x,
            y: area.y,
        };

        // Render top row
        for _ in 0..13 {
            self.render_key(keyboard::Key::None, SIZES.u1, buf);
        }
        self.render_key(keyboard::Key::None, SIZES.u2, buf);
        self.next_row(&area);

        match self.layout {
            AnyKeyboardLayout::Iso(layout) => self.render_iso(layout, area, buf),
            AnyKeyboardLayout::Ansi(layout) => self.render_ansi(layout, area, buf),
        }

        // Render bottom row
        self.next_row(&area);
        for _ in 0..3 {
            self.render_key(keyboard::Key::None, SIZES.u1_25, buf);
        }

        let space_size = Size::new(BOARD_SIZE.width - 7 * SIZES.u1_25.width, KEY_HEIGHT);
        self.render_key(keyboard::Key::None, space_size, buf);

        for _ in 0..4 {
            self.render_key(keyboard::Key::None, SIZES.u1_25, buf);
        }
    }

    fn render_iso(&mut self, layout: &IsoKeyboardLayout, area: Rect, buf: &mut Buffer) {
        // Render first key row
        self.render_key(keyboard::Key::None, SIZES.u1_5, buf);
        for sym in layout.row0 {
            self.render_key(sym, SIZES.u1, buf);
        }

        let iso_enter_width = self.remaining_key_size(area).width;
        let iso_enter = IsoEnter {
            width: iso_enter_width,
            line_height: KEY_HEIGHT,
            inset: 2,
            color: self.colors.placeholder,
        };
        iso_enter.render(
            Rect::new(
                self.origin.x,
                self.origin.y,
                iso_enter_width,
                KEY_HEIGHT * 2,
            ),
            buf,
        );

        self.next_row(&area);

        // Render second key row
        self.render_key(keyboard::Key::None, SIZES.u1_75, buf);
        for sym in layout.row1 {
            self.render_key(sym, SIZES.u1, buf);
        }
        self.next_row(&area);

        // Render third key row
        self.render_key(keyboard::Key::None, SIZES.u1_25, buf);
        for sym in layout.row2 {
            self.render_key(sym, SIZES.u1, buf);
        }
        let rshift_size = self.remaining_key_size(area);
        self.render_key(keyboard::Key::None, rshift_size, buf);
    }

    fn render_ansi(&mut self, layout: &AnsiKeyboardLayout, area: Rect, buf: &mut Buffer) {
        // Render first key row
        self.render_key(keyboard::Key::None, SIZES.u1_5, buf);
        let (last, rest) = layout.row0.split_last().expect("cannot be empty");
        for sym in rest {
            self.render_key(*sym, SIZES.u1, buf);
        }
        let last_key_size = self.remaining_key_size(area);
        self.render_key(*last, last_key_size, buf);
        self.next_row(&area);

        // Render second key row
        self.render_key(keyboard::Key::None, SIZES.u1_75, buf);
        for sym in layout.row1 {
            self.render_key(sym, SIZES.u1, buf);
        }
        let enter_size = self.remaining_key_size(area);
        self.render_key(keyboard::Key::None, enter_size, buf);
        self.next_row(&area);

        // Render third key row
        self.render_key(keyboard::Key::None, SIZES.u2_25, buf);
        for sym in layout.row2 {
            self.render_key(sym, SIZES.u1, buf);
        }
        let rshift_size = self.remaining_key_size(area);
        self.render_key(keyboard::Key::None, rshift_size, buf);
    }

    fn render_key(&mut self, key: keyboard::Key, size: Size, buf: &mut Buffer) {
        let area = Rect::new(self.origin.x, self.origin.y, size.width, size.height);

        Key::new(key, self.colors).render(area, buf);

        self.origin.x += size.width;
    }

    fn next_row(&mut self, area: &Rect) {
        self.origin.x = area.x;
        self.origin.y += SIZES.u1.height;
    }

    fn remaining_key_size(&self, area: Rect) -> Size {
        let width = BOARD_SIZE.width - (self.origin.x - area.left());
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

        self.render_layout(area, buf);
    }
}

struct Key {
    text: Option<String>,
    color: Color,
}

impl Key {
    pub fn new(key: keyboard::Key, colors: &ColorPalette) -> Self {
        let (text, color) = if let keyboard::Key::Char(sym, finger) = key {
            (
                Some(sym.to_string()),
                match finger {
                    keyboard::Finger::Pinky => colors.fingers.pinky,
                    keyboard::Finger::Ring => colors.fingers.ring,
                    keyboard::Finger::Middle => colors.fingers.middle,
                    keyboard::Finger::Index => colors.fingers.index,
                    keyboard::Finger::Thumb => colors.fingers.thumb,
                },
            )
        } else {
            (None, colors.placeholder)
        };
        Self { text, color }
    }
}

impl Widget for Key {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(self.color)
            .render(area, buf);

        if let Some(text) = self.text {
            let text_x = area.x + ((area.width - 2 /* Border */ - (text.len() as u16)) / 2) + 1 /* Border */;
            let text_y = area.y + (area.height - 1/* Line Height */) / 2;

            Text::raw(&text)
                .style(self.color)
                .render(Rect::new(text_x, text_y, text.len() as u16, 1), buf);
        }
    }
}

struct KeySizes {
    pub u1: Size,
    pub u1_25: Size,
    pub u1_5: Size,
    pub u1_75: Size,
    pub u2: Size,
    pub u2_25: Size,
}

#[derive(Debug, Default, Clone, Copy)]
struct Origin {
    pub x: u16,
    pub y: u16,
}
