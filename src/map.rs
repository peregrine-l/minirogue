use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::*, tiles::TileTextureIndex};
use rand::prelude::*;

pub mod tile_mapping;

use crate::components::{TileMarker, TilemapBuilder, TilemapMarker};
use crate::map::tile_mapping::CANARI_TILES;

const ASSET_PACK: &'static str = "OneBitCanariPack";
const TERRAIN: &'static str = "tileset/PixelPackTopDown1Bit.png";
const CHARACTERS: &'static str = "sprites/heroes/spritesheets/adventurer_idle_d.png";

pub fn setup_tilemap_builders(mut commands: Commands) {
    let terrain_tilemap = TilemapBuilder {
        asset_path: format!("{ASSET_PACK}/{TERRAIN}"),
        tilemap_marker: TilemapMarker::TerrainTilemap,
        tile_marker: TileMarker::TerrainTile,
        layer_z: 0.0,
        init_tile_fn: random_ground,
    };

    let characters_tilemap = TilemapBuilder {
        asset_path: format!("{ASSET_PACK}/{CHARACTERS}"),
        tilemap_marker: TilemapMarker::CharactersTilemap,
        tile_marker: TileMarker::CharactersTile,
        layer_z: 2.0,
        init_tile_fn: || TileTextureIndex(1),
    };

    commands.spawn(terrain_tilemap);
    commands.spawn(characters_tilemap);
}

pub fn build_tilemaps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tilemap_builder_q: Query<&TilemapBuilder>,
) {
    // Metadata, common to all tilemaps
    let map_size = TilemapSize { x: 32, y: 32 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    // Retrieve the builders from the query
    for tilemap_builder in tilemap_builder_q.iter() {
        let texture_handle: Handle<Image> =
            asset_server.load(format!("{}", tilemap_builder.asset_path));
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
                            texture_index: (tilemap_builder.init_tile_fn)(),
                            ..Default::default()
                        },
                        tilemap_builder.tile_marker.clone(),
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
                    tilemap_builder.layer_z,
                ), // bottom layer
                ..Default::default()
            },
            tilemap_builder.tilemap_marker.clone(),
        ));
    }
}

fn random_ground() -> TileTextureIndex {
    let mut rng = rand::thread_rng();

    let ground_tiles = [
        (CANARI_TILES.get("black"), 20),
        (CANARI_TILES.get("ground: 1 dot a"), 3),
        (CANARI_TILES.get("ground: 1 dot b"), 3),
        (CANARI_TILES.get("ground: 2 dots"), 2),
        (CANARI_TILES.get("ground: 3 dots a"), 1),
        (CANARI_TILES.get("ground: 3 dots b"), 1),
    ];
    let coordinates = *ground_tiles
        .choose_weighted(&mut rng, |item| item.1)
        .unwrap()
        .0
        .unwrap();
    TileTextureIndex((coordinates.0 as u32) + 16 * (coordinates.1 as u32))
}
