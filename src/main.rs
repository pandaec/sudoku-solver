mod tests;

use std::str::FromStr;

use sudoku_solver::board::Board;

fn main() {
    println!("Hello, world!");
    let board = Board::from_str("8.4.71.9.976.3....5.196....3.7495...692183...4.5726..92483591..169847...753612984").unwrap();
    println!("{:?}", board);
    println!("{:?}", board.get_at(1,1));
}
