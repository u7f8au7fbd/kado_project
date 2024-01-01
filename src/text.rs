use crate::GameState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
#[derive(Serialize, Deserialize)]
struct TextConfig {
    text: String,
    next: usize,
}

#[derive(Serialize, Deserialize)]
struct TextList {
    text_config: Vec<TextConfig>,
}

#[derive(Component)]
pub struct TextObject;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let default_style = TextStyle {
        font: asset_server.load("./font/JF-Dot-Shinonome14.ttf"),
        font_size: 50.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextObject,
        TextBundle::from_sections([TextSection::new("", default_style.clone())]).with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(560.0),
                left: Val::Px(100.0),
                max_width: Val::Px(1080.),
                max_height: Val::Px(265.),
                ..default()
            },
        ),
    ));
}

pub fn text_system(
    mut text_query: Query<(&mut Text, &mut Transform), With<TextObject>>,
    keyboard: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    let mut file = File::open("./assets/text.toml").unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).unwrap();
    let text_list: TextList = toml::from_str(&toml_str).unwrap();
    for (mut text, mut visible) in text_query.iter_mut() {
        visible.scale = Vec3::new(0., 0., 0.);
        if game_state.battle {
        } else {
            visible.scale = Vec3::new(1., 1., 0.);
            if keyboard.just_pressed(KeyCode::Space) {
                if text_list.text_config.len() > 1 && game_state.text_num != 0 {
                    let config = &text_list.text_config[game_state.text_num];
                    text.sections[0].value = config.text.to_string();
                    game_state.text_num = config.next;
                } else {
                    game_state.battle = true;
                    game_state.fase += 1;
                }
            }
        }
    }
}
