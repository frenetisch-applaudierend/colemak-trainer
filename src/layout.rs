use std::collections::{HashMap, HashSet};

pub mod qwerty;
pub mod qwertz;

pub enum KeyboardLayouts {
    Iso {
        source: IsoKeyboardLayout,
        target: IsoKeyboardLayout,
    },
    Ansi {
        source: AnsiKeyboardLayout,
        target: AnsiKeyboardLayout,
    },
}

impl KeyboardLayouts {
    pub fn allowed_target_letters(&self, level: Level) -> HashSet<char> {
        let (indices, row0, row1, row2) = match self {
            KeyboardLayouts::Iso {
                source: _,
                target: layout,
            } => (
                KeyIndices::iso(level),
                &layout.row0 as &[Key],
                &layout.row1 as &[Key],
                &layout.row2 as &[Key],
            ),
            KeyboardLayouts::Ansi {
                source: _,
                target: layout,
            } => (
                KeyIndices::ansi(level),
                &layout.row0 as &[Key],
                &layout.row1 as &[Key],
                &layout.row2 as &[Key],
            ),
        };
        let mut letters = HashSet::with_capacity(indices.size());

        for i in indices.row0 {
            match row0[*i as usize] {
                Key::Char(ch) => {
                    letters.insert(ch.to_ascii_lowercase());
                }
                _ => {}
            }
        }

        for i in indices.row1 {
            match row1[*i as usize] {
                Key::Char(ch) => {
                    letters.insert(ch.to_ascii_lowercase());
                }
                _ => {}
            }
        }

        for i in indices.row2 {
            match row2[*i as usize] {
                Key::Char(ch) => {
                    letters.insert(ch.to_ascii_lowercase());
                }
                _ => {}
            }
        }

        letters
    }

    pub fn target_layout(&self) -> AnyKeyboardLayout {
        match self {
            KeyboardLayouts::Iso { source: _, target } => AnyKeyboardLayout::Iso(target),
            KeyboardLayouts::Ansi { source: _, target } => AnyKeyboardLayout::Ansi(target),
        }
    }
}

pub struct IsoKeyboardLayout {
    pub row0: [Key; 12],
    pub row1: [Key; 12],
    pub row2: [Key; 11],
}

pub struct AnsiKeyboardLayout {
    pub row0: [Key; 13],
    pub row1: [Key; 11],
    pub row2: [Key; 10],
}

pub enum AnyKeyboardLayout<'a> {
    Iso(&'a IsoKeyboardLayout),
    Ansi(&'a AnsiKeyboardLayout),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

struct KeyIndices {
    pub row0: &'static [u8],
    pub row1: &'static [u8],
    pub row2: &'static [u8],
}

impl KeyIndices {
    pub fn iso(level: Level) -> Self {
        const ROW0: [&'static [u8]; 6] = [
            &[],
            &[],
            &[1, 2, 7, 8],
            &[1, 2, 3, 6, 7, 8],
            &[1, 2, 3, 4, 6, 7, 8],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8],
        ];
        const ROW1: [&'static [u8]; 6] = [
            &[0, 1, 2, 3, 6, 7, 8, 9],
            &[0, 1, 2, 3, 6, 7, 8, 9],
            &[0, 1, 2, 3, 6, 7, 8, 9],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        ];
        const ROW2: [&'static [u8]; 6] = [
            &[],
            &[3, 7],
            &[3, 7],
            &[3, 7],
            &[2, 3, 4, 6, 7],
            &[0, 1, 2, 3, 4, 6, 7],
        ];

        let idx = level as usize;
        Self {
            row0: ROW0[idx],
            row1: ROW1[idx],
            row2: ROW2[idx],
        }
    }

    pub fn ansi(level: Level) -> Self {
        const ROW0: [&'static [u8]; 6] = [
            &[],
            &[],
            &[1, 2, 7, 8],
            &[1, 2, 3, 6, 7, 8],
            &[1, 2, 3, 4, 6, 7, 8],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8],
        ];
        const ROW1: [&'static [u8]; 6] = [
            &[0, 1, 2, 3, 6, 7, 8, 9],
            &[0, 1, 2, 3, 6, 7, 8, 9],
            &[0, 1, 2, 3, 6, 7, 8, 9],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        ];
        const ROW2: [&'static [u8]; 6] = [
            &[],
            &[2, 6],
            &[2, 6],
            &[2, 6],
            &[1, 2, 3, 5, 6],
            &[0, 1, 2, 3, 4, 5, 6],
        ];

        let idx = level as usize;
        Self {
            row0: ROW0[idx],
            row1: ROW1[idx],
            row2: ROW2[idx],
        }
    }

    pub fn size(&self) -> usize {
        self.row0.len() + self.row1.len() + self.row2.len()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Key {
    None,
    Char(char),
}

pub struct LayoutMapper(HashMap<Key, Key>);

impl LayoutMapper {
    pub fn from(layouts: &KeyboardLayouts) -> Self {
        use KeyboardLayouts::*;

        match layouts {
            Iso { source, target } => Self::from_iso(source, target),
            Ansi { source, target } => Self::from_ansi(source, target),
        }
    }

    pub fn from_iso(source: &IsoKeyboardLayout, target: &IsoKeyboardLayout) -> Self {
        let mut map = HashMap::new();
        Self::map_row(source.row0, target.row0, &mut map);
        Self::map_row(source.row1, target.row1, &mut map);
        Self::map_row(source.row2, target.row2, &mut map);
        Self(map)
    }

    pub fn from_ansi(source: &AnsiKeyboardLayout, target: &AnsiKeyboardLayout) -> Self {
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
