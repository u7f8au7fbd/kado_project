use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct MainGUI {
    flame: bool,
}

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        MainGUI { flame: false },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10., 10.)),
                color: Color::BLACK,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -160.0, -1.0),
                scale: Vec3::new(31.5, 26.5, 0.0),
                ..default()
            },
            ..default()
        },
    ));
    commands.spawn((
        MainGUI { flame: true },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -160.0, -2.0),
                scale: Vec3::new(33.0, 28.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
}
//1145 /1130
pub fn change(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut stage_query: Query<(&mut Transform, &MainGUI)>,
    mut game_state: ResMut<GameState>,
) {
    for (mut stage, layer) in stage_query.iter_mut() {
        if game_state.battle && layer.flame {
            stage.scale.x = 33.;
        } else if !game_state.battle && layer.flame {
            stage.scale.x = 114.5;
        }

        if game_state.battle && !layer.flame {
            stage.scale.x = 31.5;
        } else if !game_state.battle && !layer.flame {
            stage.scale.x = 113.0;
        }
    }
    if keyboard.just_pressed(KeyCode::Tab) {
        game_state.battle = !game_state.battle;
    }
}

//1145 //1130
