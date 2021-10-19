use crate::evaluation::Evaluation;
use crate::prune::Prune;
use crate::size::Size;
use crate::tile::Tile;
use crate::trace::Trace;
use crate::turn::Turn;
use core::fmt;
use std::ops::Range;

/// Stores a board state of tic-tac-toe.
#[derive(Clone)]
pub struct Board {
    prune: Prune,
    size: Size,
    trace: Trace,
    position: Vec<Vec<Tile>>,
}

impl Board {
    /// Creates a new board with a given size.
    pub fn new(size: Size, trace: Option<&Trace>) -> Board {
        let width = size.width;
        let height = size.height;

        Board {
            prune: Prune::new(),
            size,
            trace: match trace {
                Some(t) => t.clone(),
                None => Trace::new(),
            },
            position: vec![vec![Tile::None; width]; height],
        }
    }

    /// Applies the evaluation to the prune object and updates it accordingly.
    pub fn apply(&mut self, evaluation: &Evaluation, is_o: bool) {
        self.prune.apply(&evaluation, is_o);
    }

    /// Determines if the entire board is filled, and no more legal moves can be played.
    pub fn is_filled(&self) -> bool {
        self.position
            .iter()
            .all(|row| row.iter().all(|tile| tile != &Tile::None))
    }

    /// Determines whether the move passed in is legal to play.
    pub fn is_legal(&self, turn: &Turn) -> bool {
        self.is_between(turn) && self.position[turn.y][turn.x] == Tile::None
    }

    /// Determines whether the branch just evaluated should be pruned based on the current prune values.
    pub fn is_pruning(&self, evaluation: &Evaluation, is_o: bool) -> bool {
        self.prune.is_pruning(evaluation, is_o)
    }

    /// Gets the height of the board.
    pub fn height(&self) -> usize {
        self.size.height
    }

    /// Returns the maximum depth needed to calculate the perfect game of tic-tac-toe.
    pub fn max_depth(&self) -> usize {
        self.size.max_depth()
    }

    /// Gets the width of the board.
    pub fn width(&self) -> usize {
        self.size.width
    }

    /// Returns a string representation of the current board state.
    pub fn current_position(&self) -> String {
        let mut out = String::new();

        for y in self.height_range() {
            out += "\n";

            for x in self.width_range() {
                out = format!("{} {}", out, self.position[y][x]);
            }
        }

        out
    }

    /// Clones the board, plays the move, and returns the board. Returns None if the move isn't legal.
    pub fn play(&self, turn: Turn) -> Option<Board> {
        if !self.is_legal(&turn) {
            None
        } else {
            let mut board = Board::new(self.size.clone(), Some(&self.trace));
            board.position = self.position.clone();
            board.position[turn.y][turn.x] = turn.tile.clone();
            board.trace.push(turn);
            Some(board)
        }
    }

    /// Evaluates a position and returns an evaluation object. Returns None if the game hasn't ended.
    pub fn evaluate(&self) -> Option<Evaluation> {
        if self.is_line(Tile::O) {
            Some(Evaluation::new(
                isize::MAX - self.trace.len(),
                self.trace.clone(),
            ))
        } else if self.is_line(Tile::X) {
            Some(Evaluation::new(
                isize::MIN + self.trace.len(),
                self.trace.clone(),
            ))
        } else if self.is_filled() {
            Some(Evaluation::new(0, self.trace.clone()))
        } else {
            None
        }
    }

    /// Returns all indices that can be indexed into the inner board.
    pub fn indices(&self) -> Vec<(usize, usize)> {
        self.size.indices()
    }

    /// Determines whether the Turn object is within bounds of the size of the board.
    fn is_between(&self, turn: &Turn) -> bool {
        self.size.is_between(turn)
    }

    /// Determines if a given tile matches any columns.
    fn is_column(&self, compare: Tile) -> bool {
        self.width_range().any(|index| {
            self.position
                .iter()
                .map(|row| row[index].clone())
                .all(|cell| cell == compare)
        })
    }

    /// Determines if a given tile matches any diagonal. Always returns false if the board width and height aren't the same.
    fn is_diagonal(&self, compare: Tile) -> bool {
        self.is_square()
            && (self
                .width_range()
                .all(|index| self.position[index][index] == compare)
                || self
                    .width_range()
                    .all(|index| self.position[self.width() - index - 1][index] == compare))
    }

    /// Determines if a given tile matches any line (row, column, or diagonal).
    fn is_line(&self, compare: Tile) -> bool {
        self.is_row(compare.clone())
            || self.is_column(compare.clone())
            || self.is_diagonal(compare.clone())
    }

    /// Determines if a given tile matches any rows.
    fn is_row(&self, compare: Tile) -> bool {
        self.position
            .iter()
            .any(|row| row.iter().all(|cell| cell == &compare))
    }

    /// Determines whether the width and height length are the same.
    fn is_square(&self) -> bool {
        self.width() == self.height()
    }

    /// Gets the range from 0 to the width of the board, mainly used for iterating columns.
    fn width_range(&self) -> Range<usize> {
        0..(self.width())
    }

    /// Gets the range from 0 to the height of the board, mainly used for iterating rows.
    fn height_range(&self) -> Range<usize> {
        0..(self.height())
    }

    /// Gets a clone to the board's trace.
    pub fn trace(&self) -> Trace {
        self.trace.clone()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(
            &self
                .evaluate()
                .map_or(format!("{}", self.trace), |evaluation| {
                    format!("{}", evaluation)
                }),
        )
    }
}
