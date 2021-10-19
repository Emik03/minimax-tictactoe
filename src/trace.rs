use crate::turn::Turn;
use core::fmt;

/// Encapsulates a turn vector, only giving access to what is needed.
#[derive(Clone)]
pub struct Trace {
    order: Vec<Turn>,
}

impl Trace {
    /// Creates a new instance of a move list.
    pub fn new() -> Trace {
        Trace { order: vec![] }
    }

    /// Append a move to the end of the move list.
    pub fn push(&mut self, turn: Turn) {
        self.order.push(turn);
    }

    /// Gets the length of the move list.
    pub fn len(&self) -> isize {
        self.order.len() as isize
    }

    /// Gets the first move that was played.
    pub fn get(&self, index: usize) -> Option<&Turn> {
        self.order.get(index)
    }
}

impl fmt::Display for Trace {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(
            &self
                .order
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
