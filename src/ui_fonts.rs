use bevy::prelude::*;
#[derive(Resource)]
pub struct UiFonts {
    pub regular: Handle<Font>,
    pub semibold: Handle<Font>,
    pub bold: Handle<Font>,
}

pub fn load_fonts(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(UiFonts {
        regular: assets.load("fonts/Inter-Regular.otf"),
        semibold: assets.load("fonts/Inter-SemiBold.otf"),
        bold: assets.load("fonts/Inter-Bold.otf"),
    });
}
