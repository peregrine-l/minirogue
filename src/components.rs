use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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

// Player

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub health: Health,
    pub position: TilePos,
}
