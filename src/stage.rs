use bevy::prelude::*;
#[derive(Component)]
pub struct UIMode {
    battle: bool,
    boarder: bool,
}
pub fn spawn(mut commands: Commands) {
    commands.spawn((
        UIMode {
            battle: false,
            boarder: false,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -160.0, -1.0),
                scale: Vec3::new(315.0, 265.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
    commands.spawn((
        UIMode {
            battle: false,
            boarder: true,
        },
        SpriteBundle {
            sprite: Sprite { ..default() },
            transform: Transform {
                translation: Vec3::new(0.0, -160.0, -2.0),
                scale: Vec3::new(330.0, 280.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn change(
    keyboard: Res<Input<KeyCode>>,
    mut stage_query: Query<(&mut Transform, &mut UIMode)>,
) {
    let speed = 3.;
    for (mut width, mut mode) in stage_query.iter_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            mode.battle = !mode.battle;
        };

        if mode.boarder && mode.battle && width.scale.x <= 1145. {
            width.scale.x += speed;
        } else if mode.boarder && !mode.battle && width.scale.x >= 330. {
            width.scale.x -= speed;
        }

        if !mode.boarder && mode.battle && width.scale.x <= 1130. {
            width.scale.x += speed;
        } else if !mode.boarder && !mode.battle && width.scale.x >= 315. {
            width.scale.x -= speed;
        }
    }
}
