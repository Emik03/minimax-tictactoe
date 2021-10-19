use crate::turn::Turn;

/// Encapsulates width and height into a struct.
#[derive(Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    /// Creates a new instance of size, setting the width and height.
    pub fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }

    /// Determines whether the turn object passed in is within bounds of the width and height.
    pub fn is_between(&self, turn: &Turn) -> bool {
        self.width > turn.x && self.height > turn.y
    }

    /// Get the maximum depth limit needed to play a perfect game.
    pub fn max_depth(&self) -> usize {
        self.width * self.height + 1
    }

    /// Gets a vector of indices suitable for iterating over the bounds of this size instance.
    pub fn indices(&self) -> Vec<(usize, usize)> {
        let mut indices: Vec<(usize, usize)> = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                indices.push((x, y));
            }
        }

        indices
    }
}
