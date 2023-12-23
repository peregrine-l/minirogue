use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::*, tiles::TileTextureIndex};
use rand::prelude::*;
use crate::components::{TerrainTile, TerrainTilemap};

pub mod tile_mapping;
use crate::map::tile_mapping::CANARI_TILES;

const ASSET_PACK: &str = "OneBitCanariPack";
const TERRAIN: &str = "tileset/PixelPackTopDown1Bit.png";


pub fn build_terrain_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
     // Metadata
     let map_size = TilemapSize { x: 32, y: 32 };
     let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
     let grid_size = tile_size.into();
 
     // Characters' layer
     let texture_handle: Handle<Image> = asset_server.load(format!("{ASSET_PACK}/{TERRAIN}"));
     let mut tile_storage = TileStorage::empty(map_size);
     let tilemap_entity = commands.spawn_empty().id();
 
     for x in 0..map_size.x {
         for y in 0..map_size.y {
             let tile_pos = TilePos { x, y };
             let tile_entity = commands
                 .spawn((TileBundle {
                     position: tile_pos,
                     tilemap_id: TilemapId(tilemap_entity),
                     texture_index: random_ground(),
                     ..Default::default()
                     },
                     TerrainTile))
                 .id();
             tile_storage.set(&tile_pos, tile_entity);
         }
     }
 
     let map_type = TilemapType::Square;
     commands
         .entity(tilemap_entity)
             .insert((TilemapBundle {
                 grid_size,
                 map_type,
                 size: map_size,
                 storage: tile_storage,
                 texture: TilemapTexture::Single(texture_handle),
                 tile_size,
                 spacing: TilemapSpacing { x: 0.0, y: 0.0 },
                 transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0), // bottom layer
                 ..Default::default()
                 },
                 TerrainTilemap));
}


fn random_ground() -> TileTextureIndex {
    let mut rng = rand::thread_rng();

    let ground_tiles = [
        (CANARI_TILES.get("black"),            20),
        (CANARI_TILES.get("ground: 1 dot a"),  3),
        (CANARI_TILES.get("ground: 1 dot b"),  3),
        (CANARI_TILES.get("ground: 2 dots"),   2),
        (CANARI_TILES.get("ground: 3 dots a"), 1),
        (CANARI_TILES.get("ground: 3 dots b"), 1),
        ];
    let coordinates = *ground_tiles
        .choose_weighted(&mut rng, |item| { item.1 })
        .unwrap().0.unwrap();
    TileTextureIndex((coordinates.0 as u32) + 16 * (coordinates.1 as u32))
}
