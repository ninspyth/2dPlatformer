use bevy::prelude::*;

#[derive(Component)]
pub struct Player;
/* pub fn load_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(ClearColor(Color::WHITE));
    commands.spawn((SpriteBundle{
        transform: Transform::from_xyz(0.0, -110.0, 3.0),
        texture: asset_server.load("characters/Knight 2D/player_in.png"),
        ..default()
    },Player
    ));
} */
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>
) {
    for (mut transform, _player) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.0;
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn player_animate_sprite(
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

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("enemy/archer/spr_ArcherIdle_strip_NoBkg.png");
    let texture_atlas =TextureAtlas::from_grid(texture_handle, Vec2::new(40.0, 50.0), 8, 1, Some(Vec2 { x: 87.6, y:0.0 }),Some(Vec2 { x: 50.0, y: 40.0 }));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 7 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                translation: Vec3::new(0.0, -180.0, 3.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}