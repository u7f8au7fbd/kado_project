use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct MainGUI;

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        MainGUI,
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
        MainGUI,
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

pub fn change(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut stage_query: Query<&mut Transform, With<MainGUI>>,
    mut game_state: ResMut<GameState>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        game_state.battle = !game_state.battle;
    }
}

//1145 //1130
