use super::{ANSIKeyboardLayout, ISOKeyboardLayout};

pub fn iso() -> ISOKeyboardLayout {
    ISOKeyboardLayout {
        row0: ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "", ""],
        row1: ["A", "S", "D", "F", "G", "H", "J", "K", "L", "", "", ""],
        row2: ["", "Z", "X", "C", "V", "B", "N", "M", "", "", ""],
    }
}

pub fn ansi() -> ANSIKeyboardLayout {
    ANSIKeyboardLayout {
        row0: ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "", "", ""],
        row1: ["A", "S", "D", "F", "G", "H", "J", "K", "L", "", ""],
        row2: ["Z", "X", "C", "V", "B", "N", "M", "", "", ""],
    }
}
