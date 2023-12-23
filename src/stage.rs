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
                    custom_size: Some(Vec2::new(352.0, 272.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(367.0, 287.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            });
        });
}
