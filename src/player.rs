use crate::components::{Player, PlayerBundle, TileMarker, TilemapMarker};
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    prelude::*,
};

pub fn build_player(mut commands: Commands, query: Query<(&TileStorage, &TilemapMarker)>) {

    commands.spawn(PlayerBundle::default()); // player entity

    for (tile_storage, tilemap_marker) in query.iter() {
        match tilemap_marker {
            TilemapMarker::CharactersTilemap => {
                let player_start_tile = tile_storage.get(&TilePos { x: 0, y: 0 }).unwrap();
                commands
                    .entity(player_start_tile)
                        .insert(TileTextureIndex(0)); // character sprite
            },
            TilemapMarker::TerrainTilemap => {}
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<&mut TilePos, With<Player>>,
    mut tiles_q: Query<(&TilePos, &mut TileTextureIndex, &TileMarker), Without<Player>>,
    tilemap_q: Query<(&TilemapSize, &TilemapMarker)>,
) {
    let input_direction = if !keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
    {
        if keyboard_input.just_pressed(KeyCode::Left) {
            Some(SquareDirection::West)
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            Some(SquareDirection::South)
        } else if keyboard_input.just_pressed(KeyCode::Up) {
            Some(SquareDirection::North)
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            Some(SquareDirection::East)
        } else {
            None
        }
    } else {
        None
    };

    if let Some(movement_direction) = input_direction {
        let mut player_pos = player_q.get_single_mut().unwrap();

        for (tilemap_size, tilemap_marker) in tilemap_q.iter() {
            match tilemap_marker {
                TilemapMarker::CharactersTilemap => {
                    let neighboring_tiles = Neighbors::get_square_neighboring_positions(
                        &player_pos,
                        tilemap_size,
                        false,
                    );

                    if let Some(target_pos) = neighboring_tiles.get(movement_direction) {
                        // OPTIMIZATION: Isn't getting all Tiles too costly?
                        let mut tiles_found = 0;
                        for (tile_pos, mut tile_texture_index, tile_marker) in tiles_q.iter_mut() {
                            match tile_marker {
                                TileMarker::CharactersTile => {
                                    // Set TileTextureIndex of target tile to 1, currently the index of the transparent sprite
                                    if *tile_pos == *player_pos {
                                        tile_texture_index.0 = 1;
                                        tiles_found += 1;
                                    }
                                    // Set TileTextureIndex of target tile to 0, currently the index of the character sprite
                                    if *tile_pos == *target_pos { 
                                        tile_texture_index.0 = 0;
                                        tiles_found += 1;
                                    }
                                }
                                TileMarker::TerrainTile => {} // for later interactions with the terrain
                            }
                            if tiles_found == 2 { break; }
                        }
                        (player_pos.x, player_pos.y) = (target_pos.x, target_pos.y);
                    }
                }
                TilemapMarker::TerrainTilemap => {} // for later interactions with the terrain
            }
        }
    }
}
