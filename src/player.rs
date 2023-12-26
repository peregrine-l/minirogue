use crate::components::{Player, PlayerBundle, TileMarker, TilemapMarker};
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    prelude::*,
};

pub fn build_player(mut commands: Commands, query: Query<(&TileStorage, &TilemapMarker)>) {
    commands.spawn(PlayerBundle::default()); // player entity
    for (tile_storage, tilemap_marker) in query.iter() {
        if tilemap_marker == &TilemapMarker::CharactersTilemap {
            let player_start_tile = tile_storage.get(&TilePos { x: 0, y: 0 }).unwrap();
            commands
                .entity(player_start_tile)
                .insert(TileTextureIndex(0));
        } // should we instert the Player marker in all layers (3 tiles) where the player is? only the
          // CharacterTile? we'll see as we implement more systems
    }
}

pub fn player_movement(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_q: Query<(Entity, &TilePos), (With<Player>, Without<TileMarker>)>,
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
        // For now, we are only interested in modifying the characters tiles, but later,
        // we will need to get the terrain tiles so we can interact with it
        let mut map_size = None;
        let mut tile_storage = None;
        for (a_map_size, a_tile_storage, tilemap_marker) in tilemap_q.iter() {
            if tilemap_marker == &TilemapMarker::CharactersTilemap {
                map_size = Some(a_map_size);
                tile_storage = Some(a_tile_storage);
            }
        }

        // ELEGANCE ALERT: Is there a nicer way to test and use potentially uninitialized
        // variables in a loop?
        if let Some(map_size) = map_size {
            if let Some(tile_storage) = tile_storage {
                let (player_id, player_pos) = player_q.get_single().unwrap();

                let neighboring_tiles =
                    Neighbors::get_square_neighboring_positions(&player_pos, map_size, false);

                if let Some(target_pos) = neighboring_tiles.get(movement_direction) {
                    let source_tile_id = tile_storage.get(player_pos).unwrap();
                    let target_tile_id = tile_storage.get(target_pos).unwrap();
                    commands.entity(player_id).insert(target_pos.clone());
                    commands.entity(source_tile_id).insert(TileTextureIndex(1));
                    commands.entity(target_tile_id).insert(TileTextureIndex(0));
                }
            }
        }
    }
}
