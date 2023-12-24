use bevy::prelude::*;
#[derive(Component)]
pub struct Stage {
    play: bool,
}
pub fn spawn(mut commands: Commands) {
    commands
        .spawn((
            Stage { play: true },
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

pub fn change(keyboard: Res<Input<KeyCode>>, mut stage_query: Query<(&mut Sprite, &mut Stage)>) {
    for (mut width, mut mode) in stage_query.iter_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            if mode.play {
                mode.play = false;
            } else {
                mode.play = true;
            }
        };
    }
}
