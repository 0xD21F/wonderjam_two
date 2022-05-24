use colored::Colorize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Grass,
    Road,
    Water,
    Empty,
}

impl Tile {
    pub const fn is_passable(&self) -> bool {
        matches!(self.tile_type, TileType::Grass) || matches!(self.tile_type, TileType::Road)
    }

    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self.tile_type {
                TileType::Grass => "*".green(),
                TileType::Road => "=".bright_red(),
                TileType::Water => "~".blue(),
                TileType::Empty => " ".normal(),
            }
        )
    }
}
