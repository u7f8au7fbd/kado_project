use bevy::prelude::*;
pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("./textures/bg.png"),
        transform: Transform::from_xyz(0., 0., -4.),
        ..default()
    });
}
