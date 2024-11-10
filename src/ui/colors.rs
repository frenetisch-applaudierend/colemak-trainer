use ratatui::style::Color;

pub struct ColorPalette {
    pub placeholder: Color,
    pub fingers: FingerColors,
}

pub struct FingerColors {
    pub pinky: Color,
    pub ring: Color,
    pub middle: Color,
    pub index: Color,
    pub thumb: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            placeholder: Color::DarkGray,

            fingers: FingerColors {
                pinky: Color::Cyan,
                ring: Color::Magenta,
                middle: Color::Yellow,
                index: Color::Green,
                thumb: Color::Red,
            },
        }
    }
}
