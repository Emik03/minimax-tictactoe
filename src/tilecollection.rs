use crate::{size::Size, tile::Tile, turn::Turn};

/// Encapsulates a 2-dimensional plane of tiles.
#[derive(Clone)]
pub struct Position {
    tiles: Vec<Vec<Tile>>,
}

impl Position {
    /// Generates a new 2-dimensional plane based on the size given.
    pub fn new(size: Size) -> Position {
        Position {
            tiles: vec![vec![Tile::None; size.width]; size.height],
        }
    }

    /// Sets the value from the index to the value of the turn. Panics if the index is out of bounds.
    pub fn set(&self, turn: Turn) {
        self.tiles[turn.y][turn.x] = turn.tile;
    }

    /// Gets the value from the index. Panics if the index is out of bounds.
    pub fn get(&self, turn: Turn) -> Tile {
        self.tiles[turn.y][turn.x]
    }
}
