use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn load_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    //spawn a camera

    //loading the background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("background/background1.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("background/background2.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("background/background3.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("background/background4.png"),
        ..default()
    });
}


pub fn load_map(
    mut commands: Commands,
    images: Res<AssetServer>
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: images.load("my_map.ldtk"),
        transform: Transform::from_xyz(-640.0, -360.0, 0.0),
        ..default()
    });
/*     commands.spawn(LdtkWorldBundle {
        ldtk_handle: images.load("map.ldtk"),
        transform: Transform::from_xyz(-640.0, -360.0, 0.0),
        ..default()
    }); */
}

pub fn load_camara(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

}