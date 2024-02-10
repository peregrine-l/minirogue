use bevy::prelude::*;

#[derive(Resource)]
pub struct AssetPack {
    pub name: &'static str,
    pub tileset: &'static str,
    pub sprites: &'static str,
}
