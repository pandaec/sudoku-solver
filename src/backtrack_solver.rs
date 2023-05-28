use std::collections::{HashSet, VecDeque};

use crate::{
    board::{Board, Cell, CellLoc},
    solver::Solver,
};

pub struct BackTrackSolver {}

#[derive(Debug)]
struct Frame {
    pub idx: usize,
    pub choice: u8,
}

impl Solver for BackTrackSolver {
    fn solve(&self, b: &Board) -> Option<Board> {
        let mut board: Board = b.clone();
        // stack: push_front(), pop()
        let mut stack: VecDeque<Frame> = VecDeque::new();
        // v.push_front(value)

        if let Some(cell) = board.find_first_variable(0) {
            for choice in board.get_possible_moves(&CellLoc::idx(cell.idx)) {
                let frame = Frame {
                    idx: cell.idx,
                    choice,
                };
                println!("Push frame {:?}", &frame);
                stack.push_front(frame);
            }
        } else {
            return Some(board);
        }
        loop {
            match stack.pop_front() {
                Some(frame) => {
                    board.set(&CellLoc::idx(frame.idx), frame.choice);

                    match board.find_first_variable(frame.idx + 1) {
                        Some(next_cell) => {
                            let next_possible_choices =
                                board.get_possible_moves(&CellLoc::idx(next_cell.idx));
                            println!("{:?} {:?} {:?}", frame, next_cell, next_possible_choices);
                            if next_possible_choices.is_empty() {
                                // deadend; require backtrack;
                                println!("Backtrack {}", frame.idx);
                                match stack.get(0) {
                                    Some(f) => board.set_none_range(f.idx, frame.idx),
                                    None => board.set_none(&CellLoc::idx(frame.idx)),
                                }
                            } else {
                                for choice in next_possible_choices {
                                    let next_frame = Frame {
                                        idx: next_cell.idx,
                                        choice,
                                    };
                                    println!("Push frame {:?}", &next_frame);
                                    stack.push_front(next_frame);
                                }
                            }
                        }
                        None => {
                            // Done?
                            return Some(board);
                        }
                    }
                }
                None => {
                    // Run till end and no result
                    return Some(board);
                }
            }
        }
    }
}
