use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_ecs_tilemap::prelude::*;

pub mod components;
pub mod map;
pub mod player;
pub mod resources;


fn camera_setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
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
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, camera_setup)
        .add_systems(Startup, crate::map::build_terrain_tilemap)
        .add_systems(Startup, crate::player::build_characters_tilemap)
        .add_systems(PostStartup, crate::player::build_player.after(crate::player::build_characters_tilemap))
        .add_systems(Update, crate::player::player_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}