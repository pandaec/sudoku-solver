mod tests;

use std::str::FromStr;

use sudoku_solver::solver::Solver;
use sudoku_solver::{backtrack_solver::BackTrackSolver, board::Board};

fn main() {
    let board = Board::from_str(
        "8.4.71.9.976.3....5.196....3.7495...692183...4.5726..92483591..169847...753612984",
    )
    .unwrap();
    // println!("{:?}", board);

    let bt_solver = BackTrackSolver {};
    match bt_solver.solve(&board) {
        Some(solved) => {
            // let s = solved.cells.iter().flat_map(|cell| cell.value)
            // .chunks(9)

            // let s = solved.cells
            //     .chunks(9)
            //     .map(|chunk| chunk.iter().map(|cell| cell.value.unwrap_or(0).to_string()).collect::<String>())
            //     .collect::<Vec<String>>();
            // println!("{:?}", s);

            solved
                .cells
                .chunks(9)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|cell| cell.value.unwrap_or(0).to_string())
                        .collect::<String>()
                })
                .for_each(|c| println!("{}", c));
        }
        None => println!("Failed!"),
    }
}
