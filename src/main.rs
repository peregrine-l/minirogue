use bevy::{log::LogPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;

pub mod components;
pub mod map;
pub mod player;

use crate::components::TilemapMarker;

fn game_setup(
    mut commands: Commands,
    query: Query<(&TileStorage, &TilemapMarker)>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    for tilemap_metadata in crate::map::setup_tilemap_metadata().iter() {
        crate::map::build_tilemaps(&mut commands, &asset_server, tilemap_metadata);
    }
    crate::player::build_player(&mut commands, query);
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Minirogue"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin::default()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, game_setup)
        .add_systems(Update, crate::player::player_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
