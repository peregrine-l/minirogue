use bevy::prelude::*;

use crate::resources::AssetPack;

pub fn select_asset_pack() -> AssetPack {
    // TODO: here test for the presence of the OneBitCanari asset pack
    // if absent, use the Free asset pack
    let free_asset_pack = AssetPack {
        name: "Free asset pack",
        tileset: "FreeAssetPack/RogueYun_SomethingBoxy.png",
        sprites: "FreeAssetPack/RogueYun_SomethingBoxy.png",
    };

    return free_asset_pack;
}

pub fn game_setup(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    for tilemap_metadata in crate::map::setup_tilemap_metadata(asset_pack).iter() {
        crate::map::build_tilemap(&mut commands, &asset_server, tilemap_metadata);
    }

    // player creation done in build_tilemap for TilemapMarker::CharactersTilemap
}