use bevy::asset::AssetPath;
use bevy::prelude::{default, AssetServer, ChildBuilder, ImageBundle, Style, UiImage, UiRect, Val};

pub fn icon(
    parent: &mut ChildBuilder,
    assets: &AssetServer,
    path: AssetPath,
    size: f32,
) {
    parent.spawn(ImageBundle {
        style: Style {
            width: Val::Px(size),
            height: Val::Px(size),
            margin: UiRect::right(Val::Px(8.0)),
            ..default()
        },
        image: UiImage::new(assets.load(path)),
        ..default()
    });
}
