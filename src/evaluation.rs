use crate::board::Board;
use crate::tile::Tile;
use crate::trace::Trace;
use crate::turn::Turn;
use ptree::{print_tree, TreeBuilder};
use std::fmt;

/// Encapsulates a result which stores which side is winning, and the move trace that it evaluated.
pub struct Evaluation {
    pub result: isize,
    trace: Trace,
}

impl Evaluation {
    /// Creates a new instance of an evaluation, encapsulating the parameters given.
    pub fn new(result: isize, trace: Trace) -> Evaluation {
        Evaluation { result, trace }
    }

    /// Takes a board and performs an alpha-beta pruned minimax algorithm to return an evaluation.
    pub fn game(board: &mut Board, max_depth: Option<usize>, is_printing: bool) -> Evaluation {
        let mut tree = TreeBuilder::new(board.current_position());
        let max_depth = max_depth.unwrap_or(board.max_depth());

        let game = Evaluation::game_inner(board, &mut tree, is_printing, false, max_depth);
        if is_printing {
            print_tree(&tree.build()).ok().unwrap();
        }

        game
    }

    /// Gets the first move that was played.
    pub fn get(&self, index: usize) -> Option<&Turn> {
        self.trace.get(index)
    }

    /// Returns the best evaluation for the respective player. Panics if the vector is empty.
    fn by_result(evaluations: Vec<Evaluation>, is_o: bool) -> Evaluation {
        let evaluations = evaluations.into_iter();
        (if is_o {
            evaluations.max_by_key(|e| e.result)
        } else {
            evaluations.min_by_key(|e| e.result)
        })
        .unwrap()
    }
    /// The inner game method that stores the current state of the game.
    fn game_inner(
        board: &mut Board,
        tree: &mut TreeBuilder,
        is_printing: bool,
        is_o: bool,
        depth: usize,
    ) -> Evaluation {
        if is_printing {
            tree.begin_child(format!("{}", board));
        }
        if let Some(evaluation) = board.evaluate() {
            if is_printing {
                tree.end_child();
            }

            evaluation
        } else if depth == 0 {
            if is_printing {
                tree.end_child();
            }

            Evaluation::new(0, board.trace())
        } else {
            let mut moves: Vec<Evaluation> = vec![];

            let tile = Tile::from_bool(is_o);
            for (x, y) in board.indices() {
                if let Some(mut evaluate) = board.play(Turn::new(x, y, tile.clone())) {
                    let game =
                        Evaluation::game_inner(&mut evaluate, tree, is_printing, !is_o, depth - 1);
                    if board.is_pruning(&game, is_o) {
                        break;
                    }

                    board.apply(&game, is_o);

                    moves.push(game);
                }
            }
            if is_printing {
                tree.end_child();
            }
            Evaluation::by_result(moves, is_o)
        }
    }
}

impl fmt::Display for Evaluation {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!("{} = {}", &self.trace, &self.result))
    }
}
