use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::*, tiles::TileTextureIndex};

use crate::components::{TileMarker, TilemapMarker};
use crate::mappings::cp437_tile;
use crate::resources::AssetPack;

pub struct TilemapMetadata {
    pub asset_path: String,
    pub tilemap_marker: TilemapMarker,
    pub tile_marker: TileMarker,
    pub layer_z: f32,
    pub init_tile: TileTextureIndex,
}

pub fn setup_tilemap_metadata(asset_pack: Res<AssetPack>) -> [TilemapMetadata; 2] {
    let terrain_tilemap = TilemapMetadata {
        asset_path: asset_pack.tileset.to_string(),
        tilemap_marker: TilemapMarker::TerrainTilemap,
        tile_marker: TileMarker::TerrainTile,
        layer_z: 0.0,
        init_tile: cp437_tile(&'\u{0000}'),
    };

    let characters_tilemap = TilemapMetadata {
        asset_path: asset_pack.sprites.to_string(),
        tilemap_marker: TilemapMarker::CharactersTilemap,
        tile_marker: TileMarker::CharactersTile,
        layer_z: 2.0,
        init_tile: cp437_tile(&'\u{0000}'),
    };

    return [terrain_tilemap, characters_tilemap];
}

pub fn build_tilemap(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    tilemap_metadata: &TilemapMetadata,
) {
    // Metadata, common to all tilemaps
    let map_size = TilemapSize { x: 32, y: 32 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    let texture_handle: Handle<Image> =
        asset_server.load(format!("{}", tilemap_metadata.asset_path));
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: tilemap_metadata.init_tile,
                        ..Default::default()
                    },
                    tilemap_metadata.tile_marker.clone(),
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let map_type = TilemapType::Square;
    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            spacing: TilemapSpacing { x: 0.0, y: 0.0 },
            transform: get_tilemap_center_transform(
                &map_size,
                &grid_size,
                &map_type,
                tilemap_metadata.layer_z,
            ), // bottom layer
            ..Default::default()
        },
        tilemap_metadata.tilemap_marker.clone(),
    ));
}
