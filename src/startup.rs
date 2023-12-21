use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::map::random_ground;
use crate::resources::AssetPack;

pub fn select_asset_pack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    use bevy::asset::LoadState;

    let canari_folder: &'static str = "OneBitCanariPack";
    let canari_path: &'static str = "tileset/PixelPackTopDown1Bit.png";
    let canari_pack = AssetPack {
        folder_name: canari_folder,
        tileset_path: canari_path,
        tileset_tile_size: (16.0, 16.0),
        tileset_handle: asset_server.load(format!("{canari_folder}/{canari_path}")),
    };

    match asset_server.get_load_state(canari_pack.tileset_handle.clone()) {
        LoadState::Failed => {
            // CanariPack is not installed, go on with the free Dinotype pack
            let dinotype_folder: &'static str = "Dinotype";
            let dinotype_path: &'static str = "dinotype_x2.png";
            let dinotype_pack = AssetPack {
                folder_name: dinotype_folder,
                tileset_path: dinotype_path,
                tileset_tile_size: (12.0, 16.0),
                tileset_handle: asset_server.load(format!("{dinotype_folder}/{dinotype_path}")),
            };
            commands.insert_resource(dinotype_pack);
        }
        _ => { commands.insert_resource(canari_pack) } // Go on with CanariPack
    }
}

pub fn startup(
    mut commands: Commands, 
    asset_pack:   Res<AssetPack>,
) {
    commands.spawn(Camera2dBundle::default());
    build_tilemap(commands, asset_pack);
}

fn build_tilemap(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
) {
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
                    texture_index: random_ground(&asset_pack),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: asset_pack.tileset_tile_size.0, y: asset_pack.tileset_tile_size.1 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;
    commands
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(asset_pack.tileset_handle.clone()),
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