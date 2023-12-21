use bevy::prelude::*;

#[derive(Resource)]
pub struct AssetPack {
    pub folder_name: &'static str,
    pub tileset_path: &'static str,
    pub tileset_tile_size: (f32, f32),
    pub tileset_handle: Handle<Image>,
}
