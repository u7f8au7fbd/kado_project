use bevy::prelude::*;

#[derive(Component)]
pub struct Barrage;
pub fn spawn(mut commands: Commands) {
    commands.spawn((
        Barrage,
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}
