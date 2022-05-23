use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

/// Tile size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    /// Fixed tile size
    Fixed(f32),
}

/// Map position customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileMapPosition {
    /// Centered tilemap
    Centered { offset: Vec3 },
    /// Custom position
    Custom(Vec3),
}

/// Tile map generation options. Must be used as a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMapOptions {
    /// Tile map size
    pub map_size: (u16, u16),
    /// Map world position
    pub position: TileMapPosition,
    /// Tile world size
    pub tile_size: TileSize,
}

impl Default for TileMapPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for TileMapOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            position: Default::default(),
            tile_size: Default::default(),
        }
    }
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Fixed(1.0)
    }
}
