pub struct TilemapPlugin;
pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::tile_map::TileMap;
use resources::TileMapOptions;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_map);
        log::info!("Loaded tile map plugin");
    }
}

impl TilemapPlugin {
    /// System to generate the complete map
    pub fn create_map(mut commands: Commands, map_options: Option<Res<TileMapOptions>>) {
        let options = match map_options {
            None => TileMapOptions::default(), // If no options is set we use the default ones
            Some(options) => options.clone(),
        };
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.create_road();
        log::info!("{}", tile_map.console_output());
    }
}
