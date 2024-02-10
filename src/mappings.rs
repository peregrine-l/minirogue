use bevy_ecs_tilemap::tiles::TileTextureIndex;

use self::cp437::CP437;

pub mod canaripack;
pub mod cp437;

pub fn cp437_tile(c: &char) -> TileTextureIndex {
    return TileTextureIndex(*CP437.get(&c).unwrap());
}

/* fn random_ground() -> TileTextureIndex {
    let mut rng = rand::thread_rng();

    let ground_tiles = [
        (CANARI_DESC_TO_2D.get("black"), 20),
        (CANARI_DESC_TO_2D.get("ground: 1 dot a"), 3),
        (CANARI_DESC_TO_2D.get("ground: 1 dot b"), 3),
        (CANARI_DESC_TO_2D.get("ground: 2 dots"), 2),
        (CANARI_DESC_TO_2D.get("ground: 3 dots a"), 1),
        (CANARI_DESC_TO_2D.get("ground: 3 dots b"), 1),
    ];
    let coordinates = *ground_tiles
        .choose_weighted(&mut rng, |item| item.1)
        .unwrap()
        .0
        .unwrap();
    TileTextureIndex((coordinates.0 as u32) + 16 * (coordinates.1 as u32))
} */
