use bevy::prelude::*;

pub struct UiTheme;

impl UiTheme {
    pub const BG_MAIN: Color = Color::srgb(0.09, 0.09, 0.12);
    pub const BG_PANEL: Color = Color::srgb(0.12, 0.12, 0.16);
    pub const BG_CARD: Color = Color::srgb(0.15, 0.15, 0.20);

    pub const ACCENT: Color = Color::srgb(0.95, 0.75, 0.25);
    pub const TEXT: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const MUTED: Color = Color::srgb(0.65, 0.65, 0.75);
}
