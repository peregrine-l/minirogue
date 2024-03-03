use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    prelude::*,
};

use crate::components::{Health, Impassable, Player, TilemapMarker};
use crate::mappings::{cp437_pos, cp437_tile};

pub fn build_player(
    commands: &mut Commands,
    player_start_tile: Entity,
    player_start_pos: (u32, u32),
) {
    commands.spawn((
        Player, 
        Health(1), 
        TilePos { x: player_start_pos.0, y: player_start_pos.1 },
        Impassable
    )); // player entity

    commands.entity(player_start_tile).insert(cp437_tile(&'☺'));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    tilemap_q: Query<(&TilemapMarker, &TilemapSize, &TileStorage)>,
    mut tiles_q: Query<&mut TileTextureIndex>,
    mut positions_q: ParamSet<(
        Query<&mut TilePos, With<Player>>,
        Query<&TilePos, With<Impassable>>,
    )>,
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
        let mut tilemap_size: Option<&TilemapSize> = None;
        let mut chars_tile_storage: Option<&TileStorage> = None;

        for (q_tilemap_marker, q_tilemap_size, q_chars_tile_storage) in tilemap_q.iter() {
            match q_tilemap_marker {
                TilemapMarker::CharactersTilemap => {
                    tilemap_size = Some(q_tilemap_size);
                    chars_tile_storage = Some(q_chars_tile_storage);
                }
                TilemapMarker::TerrainTilemap => {} // for later interactions with the terrain
            }
        }

        let mut target_pos_x: u32 = 0;
        let mut target_pos_y: u32 = 0;

        if let Ok(player_pos) = positions_q.p0().get_single_mut() {
            let neighboring_tiles = Neighbors::get_square_neighboring_positions(
                &player_pos,
                tilemap_size.unwrap(),
                false,
            );
            let target_pos = neighboring_tiles.get(movement_direction).unwrap();
            target_pos_x = target_pos.x;
            target_pos_y = target_pos.y;
        }

        for impassable_pos in positions_q.p1().iter() {
            if (impassable_pos.x == target_pos_x) &&
               (impassable_pos.y == target_pos_y) {
                return;
            }
        }

        if let Ok(mut player_pos) = positions_q.p0().get_single_mut() {
            let neighboring_tiles = Neighbors::get_square_neighboring_positions(
                &player_pos,
                tilemap_size.unwrap(),
                false,
            );
            if let Some(target_pos) = neighboring_tiles.get(movement_direction) {
                {
                    let source_tile_id = chars_tile_storage.unwrap().get(&player_pos).unwrap();
                    let mut source_tile_texture_index =
                        tiles_q.get_mut(source_tile_id).unwrap();
                    source_tile_texture_index.0 = cp437_pos(&'\u{0000}'); // set to transparent sprite
                }

                {
                    let target_tile_id = chars_tile_storage.unwrap().get(&target_pos).unwrap();
                    let mut target_tile_texture_index =
                        tiles_q.get_mut(target_tile_id).unwrap();
                    target_tile_texture_index.0 = cp437_pos(&'☺'); // set to character sprite
                }

                (player_pos.x, player_pos.y) = (target_pos.x, target_pos.y);
            }
        }
    }
}
