use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animation(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("./textures/kado/kado.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 256.0), 16, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 15 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),

            transform: Transform {
                translation: Vec3::new(0., 200., -4.),
                scale: Vec3::new(2., 2., 0.),
                ..default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
