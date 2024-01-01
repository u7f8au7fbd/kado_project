use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct Player;
pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("./textures/player.png"),
            transform: Transform::from_xyz(0., -150., 0.),
            ..default()
        },
    ));
}
pub fn wasd(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, &Player>,
    game_state: ResMut<GameState>,
) {
    let speed = 250.;
    for mut player in player_query.iter_mut() {
        if game_state.battle {
            player.scale = Vec3::new(1., 1., 0.);
            let mut x_scala: f32 = 0.0;
            let mut y_scala: f32 = 0.0;
            if keyboard.pressed(KeyCode::W) && player.translation.y <= -45. {
                y_scala += 1.0;
            }
            if keyboard.pressed(KeyCode::A) && player.translation.x >= -140. {
                x_scala -= 1.0;
            }
            if keyboard.pressed(KeyCode::S) && player.translation.y >= -275. {
                y_scala -= 1.0;
            }
            if keyboard.pressed(KeyCode::D) && player.translation.x <= 140. {
                x_scala += 1.0;
            }
            let r_scala = (x_scala.powi(2) + y_scala.powi(2)).sqrt();
            if r_scala != 0. {
                player.translation.x += speed * x_scala / r_scala * time.delta_seconds();
                player.translation.y += speed * y_scala / r_scala * time.delta_seconds();
            }
        } else {
            player.scale = Vec3::new(0., 0., 0.);
            player.translation = Vec3::new(0., -150., 0.);
        }
    }
}
