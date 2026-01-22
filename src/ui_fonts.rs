use bevy::prelude::*;
#[derive(Resource)]
pub struct UiFonts {
    pub regular: Handle<Font>,
    pub semibold: Handle<Font>,
    pub bold: Handle<Font>,
    pub emojis: Handle<Font>,
}

pub fn load_fonts(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(UiFonts {
        regular: assets.load("fonts/NotoSans-Regular.ttf"),
        semibold: assets.load("fonts/NotoSans-SemiBold.ttf"),
        bold: assets.load("fonts/NotoSans-Bold.ttf"),
        emojis: assets.load("fonts/NotoColorEmoji_WindowsCompatible.ttf"),
    });
}
