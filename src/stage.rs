use bevy::prelude::*;
#[derive(Component)]
pub struct Stage;
pub fn spawn(mut commands: Commands) {
    commands
        .spawn((
            Stage,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(315.0, 265.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., -160., -1.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(330.0, 280.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            });
        });
}
