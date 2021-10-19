use core::fmt;

/// Contains all valid tile types for Tic-Tac-Toe.
#[derive(Clone, PartialEq)]
pub enum Tile {
    None,
    O,
    X,
}

impl Tile {
    /// Converts a boolean to O if true and X otherwise.
    pub fn from_bool(is_o: bool) -> Tile {
        if is_o {
            Tile::O
        } else {
            Tile::X
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            Tile::None => "_",
            Tile::O => "O",
            Tile::X => "X",
        })
    }
}
