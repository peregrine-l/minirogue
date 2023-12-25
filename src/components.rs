use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::*, tiles::TileTextureIndex};


// Map

#[derive(Component, Clone, PartialEq)]
pub enum TileMarker {
    TerrainTile,
    CharactersTile,
}

#[derive(Component, Clone, PartialEq)]
pub enum TilemapMarker {
    TerrainTilemap,
    CharactersTilemap,
}

#[derive(Component)]
pub struct TilemapBuilder {
    pub asset_path: String,
    pub tilemap_marker: TilemapMarker,
    pub tile_marker: TileMarker,
    pub layer_z: f32,
    pub init_tile_fn: fn() -> TileTextureIndex,
}


// Player

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    health: Health,
    position: TilePos,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            health: Health(1),
            position: TilePos { x: 0, y: 0 },
        }
    }
}
