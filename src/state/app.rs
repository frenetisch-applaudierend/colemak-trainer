use crate::layout::{qwertz, KeyboardLayouts, Level};

pub struct AppState {
    pub level: Level,
    pub layouts: KeyboardLayouts,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            level: Level::One,
            layouts: KeyboardLayouts::Iso {
                source: qwertz::iso(),
                target: qwertz::iso(),
            },
        }
    }
}
