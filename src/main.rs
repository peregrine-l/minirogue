use bevy::{log::LogPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;

pub mod components;
pub mod map;
pub mod mappings;
pub mod player;
pub mod resources;

use crate::components::TilemapMarker;
use crate::resources::AssetPack;

fn select_asset_pack() -> AssetPack {
    // TODO: here test for the presence of the OneBitCanari asset pack
    // if absent, use the Free asset pack
    let free_asset_pack = AssetPack {
        name: "Free asset pack",
        tileset: "FreeAssetPack/RogueYun_SomethingBoxy.png",
        sprites: "FreeAssetPack/RogueYun_SomethingBoxy.png",
    };

    return free_asset_pack;
}

fn game_setup(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    query: Query<(&TileStorage, &TilemapMarker)>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    for tilemap_metadata in crate::map::setup_tilemap_metadata(asset_pack).iter() {
        crate::map::build_tilemap(&mut commands, &asset_server, tilemap_metadata);
    }

    
    crate::player::build_player(&mut commands, query);
}

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
