use crate::tile::Tile;
use core::fmt;

/// Encapsulates a move in Tic-Tac-Toe.
#[derive(Clone)]
pub struct Turn {
    pub x: usize,
    pub y: usize,
    pub tile: Tile,
}

impl Turn {
    /// Creates a new instance of turn, setting x, y, and tile.
    pub fn new(x: usize, y: usize, tile: Tile) -> Turn {
        Turn { x, y, tile }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!("{}: {{ {}, {} }}", self.tile, self.x, self.y))
    }
}
