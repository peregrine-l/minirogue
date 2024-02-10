use crate::components::{Health, Player, PlayerBundle, TilemapMarker};
use crate::mappings::cp437_tile;
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    prelude::*,
};

pub fn build_player(commands: &mut Commands, query: Query<(&TileStorage, &TilemapMarker)>) {
    commands.spawn(PlayerBundle {
        marker: Player,
        health: Health(1),
        position: TilePos { x: 0, y: 0 },
    }); // player entity

    if query.is_empty() { info!("empty q"); }
    for (tile_storage, tilemap_marker) in query.iter() {
        match tilemap_marker {
            TilemapMarker::CharactersTilemap => {
                let player_start_tile = tile_storage.get(&TilePos { x: 0, y: 0 }).unwrap();
                commands.entity(player_start_tile).insert(cp437_tile(&'☺')); // character sprite
                info!("init player tile");
            }
            TilemapMarker::TerrainTilemap => {}
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<&mut TilePos, With<Player>>,
    mut tiles_q: Query<&mut TileTextureIndex>,
    tilemap_q: Query<(&TilemapSize, &TileStorage, &TilemapMarker)>,
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

        for (tilemap_size, tile_storage, tilemap_marker) in tilemap_q.iter() {
            match tilemap_marker {
                TilemapMarker::CharactersTilemap => {
                    let neighboring_tiles = Neighbors::get_square_neighboring_positions(
                        &player_pos,
                        tilemap_size,
                        false,
                    );

                    if let Some(target_pos) = neighboring_tiles.get(movement_direction) {
                        {
                            let source_tile_id = tile_storage.get(&player_pos).unwrap();
                            let mut source_tile_texture_index =
                                tiles_q.get_mut(source_tile_id).unwrap();
                            source_tile_texture_index.0 = cp437_tile(&'\u{0000}').0;
                            // set to transparent sprite
                        }

                        {
                            let target_tile_id = tile_storage.get(target_pos).unwrap();
                            let mut target_tile_texture_index =
                                tiles_q.get_mut(target_tile_id).unwrap();
                            target_tile_texture_index.0 = cp437_tile(&'☺').0; // set to character sprite
                        }

                        (player_pos.x, player_pos.y) = (target_pos.x, target_pos.y);
                    }
                }

                TilemapMarker::TerrainTilemap => {} // for later interactions with the terrain
            }
        }
    }
}
