use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::*, helpers::square_grid::neighbors::{Neighbors, SquareDirection}};
use crate::components::{CharactersTile, CharactersTilemap, PlayerBundle, Player};

const ASSET_PACK: &str = "OneBitCanariPack";
const CHARACTERS: &str = "sprites/heroes/spritesheets/adventurer_idle_d.png";

pub fn build_characters_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Metadata
    let map_size = TilemapSize { x: 32, y: 32 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    // Characters' layer
    let texture_handle: Handle<Image> = asset_server.load(format!("{ASSET_PACK}/{CHARACTERS}"));
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(1), // transparent on current CHARACTERS
                    ..Default::default()
                    },
                    CharactersTile))
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
                transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 2.0), // top layer
                ..Default::default()
                },
                CharactersTilemap));
}

pub fn build_player(
    mut commands: Commands,
    query: Query<&TileStorage, With<CharactersTilemap>>,
) {
    let tile_storage = query.get_single().unwrap();
    let player_start_tile = tile_storage.get(&TilePos {x: 15, y: 15}).unwrap();
    commands.entity(player_start_tile)
        .insert((
            PlayerBundle::default(), 
            TileTextureIndex(0),
            ));
}


pub fn player_movement(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    characters_tilemap_q: Query<(&TilemapSize, &TileStorage), With<CharactersTilemap>>, // only one
    current_player_tile_q: Query<(Entity, &TilePos), With<Player>>, // only one
    tile_texture_index_q: Query<(&TilePos, &TileTextureIndex), With<CharactersTile>>,
    // current_terrain_tile_q: to interact with terrain
) {
    let input_direction = 
    if !keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        if keyboard_input.just_pressed(KeyCode::D) { Some(SquareDirection::West) }
        else if keyboard_input.just_pressed(KeyCode::H) { Some(SquareDirection::South) }
        else if keyboard_input.just_pressed(KeyCode::T) { Some(SquareDirection::North) }
        else if keyboard_input.just_pressed(KeyCode::N) { Some(SquareDirection::East) } 
        else { None }
    } else { None };

    // Get results of queries
    let (source_tile_id, source_tilepos) = 
        current_player_tile_q.get_single().unwrap();
    let (map_size, tile_storage) = characters_tilemap_q.get_single().unwrap();

    if let Some(movement_direction) = input_direction {
        let neighboring_tiles = 
            Neighbors::get_square_neighboring_positions(source_tilepos, map_size, false);
        if let Some(destination_tilepos) = neighboring_tiles.get(movement_direction) {
            // Saving TilePos components for future swap
            let source_new_tilepos = TilePos { x: destination_tilepos.x, y: destination_tilepos.y };
            let destination_new_tilepos = TilePos { x: source_tilepos.x, y: source_tilepos.y };
            // Getting and saving TileTextureIndex components for future swap
            let mut source_index = &TileTextureIndex(0);
            let mut destination_index = &TileTextureIndex(1);
            for (tilepos, tile_texture_index) in tile_texture_index_q.iter() {
                if tilepos == source_tilepos {
                    source_index = tile_texture_index;
                }
                if tilepos == destination_tilepos {
                    destination_index = tile_texture_index;
                }
            }
            let source_new_index = TileTextureIndex(destination_index.0);
            let destination_new_index = TileTextureIndex(source_index.0);
            // Commit the swaps
            let destination_tile_id = tile_storage.get(destination_tilepos).unwrap();
            commands.entity(source_tile_id).insert(source_new_tilepos).insert(source_new_index);
            commands.entity(destination_tile_id).insert(destination_new_tilepos).insert(destination_new_index);
        }
    }
}
