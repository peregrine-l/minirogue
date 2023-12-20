use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::map::random_ground;

pub fn startup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    build_tilemap(commands, asset_server);
}

fn build_tilemap(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("OneBitCanariPack/tileset/PixelPackTopDown1Bit.png");
    let map_size = TilemapSize { x: 32, y: 32 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    // Construct tiles
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: random_ground(),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;
    commands
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            spacing: TilemapSpacing { x: 0.0, y: 0.0 },
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0) *
                       Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        });
}

/* fn walls() {
    if x == 0 || x == map_size.x - 1 || y == 0 || y == map_size.y - 1 {
        commands.entity(tile_entity)
            .insert(Impassable)
            .insert(TileTextureIndex(5));
    }
} */

/* fn player() {
    // Construct player and amend its tile, TODO: move to player builder
    let player_start_tile = tile_storage.get(&TilePos {x: 15, y: 15}).unwrap();
    commands.entity(player_start_tile)
        .insert(Player)
        .insert(Health(20))
        .insert(TileTextureIndex(1));
}
 */