pub mod qwerty;
pub mod qwertz;

pub enum KeyboardLayout {
    ISO(ISOKeyboardLayout),
    ANSI(ANSIKeyboardLayout),
}

pub struct ISOKeyboardLayout {
    pub row0: [&'static str; 12],
    pub row1: [&'static str; 12],
    pub row2: [&'static str; 11],
}

pub struct ANSIKeyboardLayout {
    pub row0: [&'static str; 13],
    pub row1: [&'static str; 11],
    pub row2: [&'static str; 10],
}
