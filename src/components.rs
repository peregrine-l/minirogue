use bevy::prelude::*;

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
pub struct Impassable;

pub enum WallOrientation {
    NorthwestCorner,
    Horizontal,
    NortheastCorner,
    Vertical,
    SoutheastCorner,
    SouthwestCorner,
    Pillar,
    Undefined,
}

#[derive(Component)]
pub struct Wall(pub WallOrientation);

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub pass: Impassable,    
}

// Player

#[derive(Component, PartialEq)]
pub struct Player;

#[derive(Component)]
pub struct Health(pub u32);
