use super::{ANSIKeyboardLayout, ISOKeyboardLayout, KeyboardLayout};

pub fn iso() -> KeyboardLayout {
    KeyboardLayout::ISO(ISOKeyboardLayout {
        row0: ["Q", "W", "E", "R", "T", "Z", "U", "I", "O", "P", "", ""],
        row1: ["A", "S", "D", "F", "G", "H", "J", "K", "L", "", "", ""],
        row2: ["", "Y", "X", "C", "V", "B", "N", "M", "", "", ""],
    })
}

pub fn ansi() -> KeyboardLayout {
    KeyboardLayout::ANSI(ANSIKeyboardLayout {
        row0: ["Q", "W", "E", "R", "T", "Z", "U", "I", "O", "P", "", "", ""],
        row1: ["A", "S", "D", "F", "G", "H", "J", "K", "L", "", ""],
        row2: ["Y", "X", "C", "V", "B", "N", "M", "", "", ""],
    })
}
