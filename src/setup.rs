pub mod setup {
    use bevy::prelude::*;
    pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2dBundle::default());
    }
}