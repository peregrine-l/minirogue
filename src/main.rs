use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod components;
pub mod map;
pub mod resources;
pub mod startup;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Minirogue"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(PreStartup, crate::startup::select_asset_pack)
        .add_systems(Startup, crate::startup::startup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}