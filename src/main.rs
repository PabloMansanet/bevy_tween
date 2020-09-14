use std::time::Duration;

use bevy::prelude::*;
mod tween;

use tween::Tween;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(tween::tween_system::<Translation>.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("icon.png").unwrap();
    let start_position = Translation::from(Vec3::new(-300.0, -300.0, 0.0));
    let end_position = Translation::from(Vec3::new(300.0, 300.0, 0.0));

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(start_position.tween_to(end_position, Duration::from_millis(500)));
}
