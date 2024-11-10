use crate::keyboard::{layouts, KeyboardLayouts, Level};

pub struct AppState {
    pub level: Level,
    pub layouts: KeyboardLayouts,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            level: Level::One,
            layouts: KeyboardLayouts::Iso {
                source: layouts::qwertz::iso(),
                target: layouts::colemak_dh::iso(),
            },
        }
    }
}
