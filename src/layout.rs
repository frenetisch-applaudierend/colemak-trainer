use std::collections::HashMap;

pub mod qwerty;
pub mod qwertz;

pub enum KeyboardLayout {
    ISO(ISOKeyboardLayout),
    ANSI(ANSIKeyboardLayout),
}

pub struct ISOKeyboardLayout {
    pub row0: [Key; 12],
    pub row1: [Key; 12],
    pub row2: [Key; 11],
}

pub struct ANSIKeyboardLayout {
    pub row0: [Key; 13],
    pub row1: [Key; 11],
    pub row2: [Key; 10],
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Key {
    None,
    Char(char),
}

pub struct LayoutMapper(HashMap<Key, Key>);

impl LayoutMapper {
    pub fn from(source: &KeyboardLayout, target: &KeyboardLayout) -> Self {
        use KeyboardLayout::*;

        match (source, target) {
            (ISO(source), ISO(target)) => Self::from_iso(source, target),
            (ANSI(source), ANSI(target)) => Self::from_ansi(source, target),
            _ => {
                panic!("LayoutMapper can only be created from layouts of the same type (ISO/ANSI)")
            }
        }
    }

    pub fn from_iso(source: &ISOKeyboardLayout, target: &ISOKeyboardLayout) -> Self {
        let mut map = HashMap::new();
        Self::map_row(source.row0, target.row0, &mut map);
        Self::map_row(source.row1, target.row1, &mut map);
        Self::map_row(source.row2, target.row2, &mut map);
        Self(map)
    }

    pub fn from_ansi(source: &ANSIKeyboardLayout, target: &ANSIKeyboardLayout) -> Self {
        let mut map = HashMap::new();
        Self::map_row(source.row0, target.row0, &mut map);
        Self::map_row(source.row1, target.row1, &mut map);
        Self::map_row(source.row2, target.row2, &mut map);
        Self(map)
    }

    fn map_row<const N: usize>(source: [Key; N], target: [Key; N], map: &mut HashMap<Key, Key>) {
        for i in 0..N {
            let source = source[i];
            let target = target[i];

            if !matches!(source, Key::None) {
                map.insert(source, target);
            }
        }
    }

    pub fn map(&self, key: Key) -> Key {
        self.0.get(&key).copied().unwrap_or(Key::None)
    }
}
