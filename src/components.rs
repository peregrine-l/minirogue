use bevy::prelude::*;

#[derive(Component)]
pub struct CharactersTile;

#[derive(Component)]
pub struct CharactersTilemap;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Impassable;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct TerrainTile;

#[derive(Component)]
pub struct TerrainTilemap;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    health: Health,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            health: Health(1),
        }
    }
}