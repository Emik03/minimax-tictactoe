use crate::evaluation::Evaluation;

/// Contains an alpha and beta value meant for minimax algorithms.
#[derive(Clone)]
pub struct Prune {
    pub alpha: isize,
    pub beta: isize,
}

impl Prune {
    /// Creates a new instance of a prune that automatically sets the alpha and beta values.
    pub fn new() -> Prune {
        Prune {
            alpha: isize::MIN,
            beta: isize::MAX,
        }
    }

    /// Applies the evaluation given and updates the Prune object according to the boolean passed in.
    pub fn apply(&mut self, evaluation: &Evaluation, is_alpha: bool) {
        if is_alpha {
            self.alpha = self.alpha.max(evaluation.result);
        } else {
            self.beta = self.beta.max(evaluation.result);
        }
    }

    /// Determines whether the current evaluation branch should be pruned based on the values of this Prune object.
    pub fn is_pruning(&self, evaluation: &Evaluation, is_alpha: bool) -> bool {
        if is_alpha {
            self.beta < evaluation.result
        } else {
            self.alpha > evaluation.result
        }
    }
}
