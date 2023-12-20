use bevy::prelude::*;

#[derive(Component)]
pub struct EntropySource;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Impassable;

#[derive(Component)]
pub struct Player;
