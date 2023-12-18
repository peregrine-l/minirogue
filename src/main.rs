use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod setup;

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
        .add_systems(Startup, crate::setup::setup::startup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}