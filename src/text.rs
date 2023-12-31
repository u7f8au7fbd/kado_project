use bevy:: prelude::*;
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
        TextBundle::from_sections([TextSection::new("TEST", default_style.clone())]).with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(560.0),
                left: Val::Px(100.0),
                max_width: Val::Px(1100.),
                max_height: Val::Px(265.),
                ..default()
            },
        ),
    ));
}

pub fn animation(
    mut timer: Local<f32>,
    time: Res<Time>,
    mut text_query: Query<&mut Text, With<TextObject>>,
) {
    let base_text: String = "＊LINE:1\n＊LINE:2\n＊LINE:3".to_string();
    let text_clone = base_text.clone();
    *timer -= time.delta_seconds();
    if *timer <= 0. {
        *timer = 3.0;
    }
    for mut text in text_query.iter_mut() {
        text.sections[0].value = text_clone.to_string();
    }
}
