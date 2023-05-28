use crate::board::Board;

pub trait Solver {
    fn solve(&self, board: &Board) -> Option<Board>;
}
