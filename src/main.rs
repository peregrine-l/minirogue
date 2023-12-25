use bevy::{log::LogPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;

pub mod components;
pub mod map;
pub mod player;

fn camera_setup(mut commands: Commands) {
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
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin::default()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, camera_setup)
        .add_systems(PreStartup, crate::map::setup_tilemap_builders)
        .add_systems(Startup, crate::map::build_tilemaps.after(crate::map::setup_tilemap_builders))
        .add_systems(PostStartup, crate::player::build_player.after(crate::map::build_tilemaps))
        .add_systems(Update, crate::player::player_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
