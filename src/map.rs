use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::components::{TileMarker, TilemapMarker, Impassable, Wall, WallOrientation};
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

pub fn build_tilemap (
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    tilemap_metadata: &TilemapMetadata,
) {
    // Metadata, common to all tilemaps
    let map_size = TilemapSize { x: 32, y: 32 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let start_position = (1, 1);

    let texture_handle: Handle<Image> =
        asset_server.load(format!("{}", tilemap_metadata.asset_path));
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let mut player_start_tile = None;
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
            if (x, y) == start_position {
                player_start_tile = Some(tile_entity);
            }
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    build_walls(commands, tilemap_entity, map_size, &mut tile_storage);

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

    // Player initialization, needs starting tile entity
    if tilemap_metadata.tilemap_marker == TilemapMarker::CharactersTilemap {
        crate::player::build_player(commands, player_start_tile.unwrap(), start_position);
    }
}

fn build_walls(
    commands: &mut Commands,
    tilemap_entity: Entity, 
    map_size: TilemapSize,
    tile_storage: &mut TileStorage
) {
    fn build_single_wall(
        commands: &mut Commands,
        tilemap_entity: Entity, 
        tile_storage: &mut TileStorage,
        x: u32,
        y: u32,
    ) {
        let tile_pos = TilePos { x, y };
        let tile_entity = commands
        .spawn((
            TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: cp437_tile(&'#'),
                ..Default::default()
            },
            TileMarker::TerrainTile,
            Wall(WallOrientation::Undefined),
            Impassable,
        ))
        .id();
        tile_storage.set(&tile_pos, tile_entity);
    }

    for i in 0..map_size.x {
        for j in 0..map_size.y {
            if j == 0 || j == map_size.y - 1 {
                build_single_wall(commands, tilemap_entity, tile_storage, i, j);
            } else if !(j == 0 || j == map_size.y - 1) 
                      && (i == 0 || i == map_size.x - 1) {
                build_single_wall(commands, tilemap_entity, tile_storage, i, j);
            }
        }
    }
}