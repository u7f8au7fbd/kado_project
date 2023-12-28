use bevy::prelude::*;
#[derive(Component)]
pub struct TextObject;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let default_style = TextStyle {
        font: asset_server.load("./font/JF-Dot-Shinonome14.ttf"),
        font_size: 55.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextObject,
        TextBundle::from_sections([TextSection::new("", default_style.clone())]).with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(540.0),
                left: Val::Px(100.0),
                max_width: Val::Px(1100.),
                max_height: Val::Px(265.),
                ..default()
            },
        ),
    ));
}

pub fn animation(mut text_query: Query<&mut Text, With<TextObject>>) {
    let base_text = String::from("＊LINE:1\n＊LINE:2\n＊LINE:3");
    let text_size: usize = base_text.chars().count();
    for i in 0..text_size {
        let get_text: String = base_text.chars().take(i).collect();
        for mut text in text_query.iter_mut() {
            text.sections[0].value = get_text.clone();
            println!("{}", get_text);
        }
    }
}
