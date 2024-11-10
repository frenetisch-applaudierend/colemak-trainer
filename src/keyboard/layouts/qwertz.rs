use crate::keyboard::{AnsiKeyboardLayout, Finger, IsoKeyboardLayout, Key};

pub fn iso() -> IsoKeyboardLayout {
    IsoKeyboardLayout {
        row0: [
            Key::Char('Q', Finger::Pinky),
            Key::Char('W', Finger::Ring),
            Key::Char('E', Finger::Middle),
            Key::Char('R', Finger::Index),
            Key::Char('T', Finger::Index),
            Key::Char('Z', Finger::Index),
            Key::Char('U', Finger::Index),
            Key::Char('I', Finger::Middle),
            Key::Char('O', Finger::Ring),
            Key::Char('P', Finger::Pinky),
            Key::None,
            Key::None,
        ],
        row1: [
            Key::Char('A', Finger::Pinky),
            Key::Char('S', Finger::Ring),
            Key::Char('D', Finger::Middle),
            Key::Char('F', Finger::Index),
            Key::Char('G', Finger::Index),
            Key::Char('H', Finger::Index),
            Key::Char('J', Finger::Index),
            Key::Char('K', Finger::Middle),
            Key::Char('L', Finger::Ring),
            Key::None,
            Key::None,
            Key::None,
        ],
        row2: [
            Key::None,
            Key::Char('Y', Finger::Pinky),
            Key::Char('X', Finger::Ring),
            Key::Char('C', Finger::Middle),
            Key::Char('V', Finger::Index),
            Key::Char('B', Finger::Index),
            Key::Char('N', Finger::Index),
            Key::Char('M', Finger::Index),
            Key::None,
            Key::None,
            Key::None,
        ],
    }
}

pub fn ansi() -> AnsiKeyboardLayout {
    AnsiKeyboardLayout {
        row0: [
            Key::Char('Q', Finger::Pinky),
            Key::Char('W', Finger::Ring),
            Key::Char('E', Finger::Middle),
            Key::Char('R', Finger::Index),
            Key::Char('T', Finger::Index),
            Key::Char('Z', Finger::Index),
            Key::Char('U', Finger::Index),
            Key::Char('I', Finger::Middle),
            Key::Char('O', Finger::Ring),
            Key::Char('P', Finger::Pinky),
            Key::None,
            Key::None,
            Key::None,
        ],
        row1: [
            Key::Char('A', Finger::Pinky),
            Key::Char('S', Finger::Ring),
            Key::Char('D', Finger::Middle),
            Key::Char('F', Finger::Index),
            Key::Char('G', Finger::Index),
            Key::Char('H', Finger::Index),
            Key::Char('J', Finger::Index),
            Key::Char('K', Finger::Middle),
            Key::Char('L', Finger::Ring),
            Key::None,
            Key::None,
        ],
        row2: [
            Key::Char('Y', Finger::Pinky),
            Key::Char('X', Finger::Ring),
            Key::Char('C', Finger::Middle),
            Key::Char('V', Finger::Index),
            Key::Char('B', Finger::Index),
            Key::Char('N', Finger::Index),
            Key::Char('M', Finger::Index),
            Key::None,
            Key::None,
            Key::None,
        ],
    }
}
