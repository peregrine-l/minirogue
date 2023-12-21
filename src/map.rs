use bevy::prelude::*;
use rand::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;
use crate::map::tiles::CANARI_TILES;
use crate::resources::AssetPack;
use self::tiles::DINOTYPE_TILES;
pub mod tiles;


pub fn random_ground(pack: &Res<AssetPack>) -> TileTextureIndex {
    let mut rng = rand::thread_rng();
    let index: u32;
    
    if pack.folder_name == "Dinotype" {
        let ground_tiles = [
            (DINOTYPE_TILES.get(&' '), 10),
            (DINOTYPE_TILES.get(&'.'), 1)
            ];
        index = 1; /* *ground_tiles
            .choose_weighted(&mut rng, |item| { item.1 })
            .unwrap().0.unwrap(); */
    } else if pack.folder_name == "OneBitCanariPack" {
        let ground_tiles = [
            (CANARI_TILES.get("black"),            20),
            (CANARI_TILES.get("ground: 1 dot a"),  3),
            (CANARI_TILES.get("ground: 1 dot b"),  3),
            (CANARI_TILES.get("ground: 2 dots"),   2),
            (CANARI_TILES.get("ground: 3 dots a"), 1),
            (CANARI_TILES.get("ground: 3 dots b"), 1),
            ];
        let coordinates = *ground_tiles
            .choose_weighted(&mut rng, |item| { item.1 })
            .unwrap().0.unwrap();
        index = (coordinates.0 as u32) + 16 * (coordinates.1 as u32);
    } else { return TileTextureIndex(0); }
    TileTextureIndex(index)
}
