use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod components;
pub mod map;
pub mod mappings;
pub mod player;
pub mod resources;
pub mod setup;

use bevy::log::LogPlugin;
use crate::setup::select_asset_pack;
use crate::setup::game_setup;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
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
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "wgpu=warn,bevy_ecs=info".to_string(),
                    ..default()
                }),
        )
        .add_plugins(TilemapPlugin)
        .insert_resource(select_asset_pack())
        .add_systems(Startup, game_setup)
        .add_systems(Update, crate::player::player_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
