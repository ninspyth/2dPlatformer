use bevy::prelude::*;

const PLAYER_SPEED: f32 = 150.0;

#[derive(Component)]
pub struct Player {
    vel: Vec2
}

impl Default for Player {
    fn default() -> Self {
        Player {
            vel: Vec2::new(0.0, 0.0)
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

pub enum PlayerState {
    Attack,
    Idle,
    Jump,
    Death,
    Roll,
    Run,
    Sheild
}

pub fn animate_player(
    time: Res<Time>,
    mut player_q: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>
) {
    for (idx, mut timer, mut sprite) in player_q.iter_mut(){
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == idx.last {
                idx.first
            }
            else {
                sprite.index + 1
            }
        }
    }
}


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("characters/Knight/noBKG_KnightIdle_strip.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 44.0), 15, 1, Some(Vec2::new(40.0, 0.0)), Some(Vec2::new(23.0, 14.0)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices {
        first: 1,
        last: 14
    };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                translation: Vec3::new(-600.0, -248.0, 100.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        },
        Player::default(),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    ));
}

pub fn player_movement(
    mut player_q: Query<&mut Transform, With<Player>>,
    mut player_vel: Query<&mut Player>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut player_transfrom = player_q.single_mut();
    let mut player_vel = player_vel.single_mut();

    //forward
    if input.pressed(KeyCode::D) {
        player_vel.vel.x = 1.0;
        player_transfrom.translation.x += player_vel.vel.x * PLAYER_SPEED * time.delta_seconds();
    }
}