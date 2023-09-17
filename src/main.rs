mod world;
mod player;

use bevy::prelude::*;
use::bevy_ecs_ldtk::prelude::*;
use world::*;
use player::*;

fn main() {
    App::new()
        .insert_resource(LevelSelection::Index(0))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Knight Rush".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, (
            load_background,
            load_map,
            load_camera,
            spawn_player
        ))
        .add_systems(Update, (
            animate_player,
            player_movement
        ))
        .run()
}
