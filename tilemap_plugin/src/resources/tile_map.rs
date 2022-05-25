use crate::components::Coordinates;
use crate::resources::tile::Tile;
use bresenham::*;
use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut};

use super::tile::TileType;

/// Base tile map
#[derive(Debug, Clone)]
pub struct TileMap {
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Generates an empty grass map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| {
                (0..width)
                    .into_iter()
                    .map(|_| Tile {
                        tile_type: TileType::Grass,
                    })
                    .collect()
            })
            .collect();
        Self { height, width, map }
    }

    pub fn console_output(&self) -> String {
        let mut buffer = format!("Map ({}, {}):\n", self.width, self.height);
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    // Getter for `width`
    pub fn width(&self) -> u16 {
        self.width
    }

    // Getter for `height`
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn is_passable_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        };
        self.map[coordinates.y as usize][coordinates.x as usize].is_passable()
    }

    pub fn create_road(&mut self) {
        let mut rng = thread_rng();

        // y-axis = bottom(0)-top(self.height-1)
        // x-axis = left(0)-right(self.width-1)

        // Pick point on top edge
        let (start_y, start_x) = (self.height() - 1, rng.gen_range(0..self.width()));

        // Pick point on bottom edge
        let (end_y, end_x) = (0u16, rng.gen_range(0..self.width()));

        self[start_y as usize][start_x as usize] = Tile {
            tile_type: TileType::Road,
        };
        self[end_y as usize][end_x as usize] = Tile {
            tile_type: TileType::Road,
        };

        // Run twice for double thickness road?
        for (y, x) in Bresenham::new(
            (start_y as isize, start_x as isize),
            (end_y as isize, end_x as isize),
        ) {
            self[y as usize][x as usize] = Tile {
                tile_type: TileType::Road,
            };

            // ????????????? this sucks ?????????
            let x_1 = x + 1;
            if (x_1 >= 0 && x_1 <= self.width() as isize) {
                self[y as usize][x_1 as usize] = Tile {
                    tile_type: TileType::Road,
                };
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
