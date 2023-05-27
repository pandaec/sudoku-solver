#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr};

    use sudoku_solver::board::{Board, CellLoc};

    fn get_board_a() -> Board {
        // 8.4 .71 .9.
        // 976 .3. ...
        // 5.1 96. ...

        // 3.7 495 ...
        // 692 183 ...
        // 4.5 726 ..9

        // 248 359 1..
        // 169 847 ...
        // 753 612 984
        Board::from_str(
            "8.4.71.9.976.3....5.196....3.7495...692183...4.5726..92483591..169847...753612984",
        )
        .unwrap()
    }

    #[test]
    fn board_get_at() {
        let board = get_board_a();
        assert_eq!(Some(8), board.get_at(0, 0));
        assert_eq!(Some(4), board.get_at(3, 3));
        assert_eq!(Some(9), board.get_at(5, 7));
        assert_eq!(None, board.get_at(6, 7));
        assert_eq!(None, board.get_at(7, 2));
    }

    #[test]
    fn board_get() {
        let board = get_board_a();
        assert_eq!(Some(8), board.get(&CellLoc::new(0, 0)));
        assert_eq!(Some(4), board.get(&CellLoc::new(3, 3)));
        assert_eq!(Some(9), board.get(&CellLoc::new(5, 7)));
        assert_eq!(None, board.get(&CellLoc::new(6, 7)));
        assert_eq!(None, board.get(&CellLoc::new(7, 2)));
    }

    #[test]
    fn get_row_values() {
        let board = get_board_a();
        assert_eq!(
            HashSet::from([5, 1, 9, 6]),
            board.get_row_values(&CellLoc::new(2, 2))
        );
        assert_eq!(
            HashSet::from([6, 9, 2, 1, 8, 3]),
            board.get_row_values(&CellLoc::new(4, 4))
        );
        assert_eq!(
            HashSet::from([2, 4, 8, 3, 5, 9, 1]),
            board.get_row_values(&CellLoc::new(6, 6))
        );
        assert_eq!(
            HashSet::from_iter(1..=9),
            board.get_row_values(&CellLoc::new(8, 8))
        );
    }

    #[test]
    fn get_col_values() {
        let board = get_board_a();
        assert_eq!(
            HashSet::from_iter(1..=9),
            board.get_col_values(&CellLoc::new(2, 2))
        );
        assert_eq!(
            HashSet::from_iter(1..=9),
            board.get_col_values(&CellLoc::new(4, 4))
        );
        assert_eq!(
            HashSet::from([1,9]),
            board.get_col_values(&CellLoc::new(6, 6))
        );
        assert_eq!(
            HashSet::from([4,9]),
            board.get_col_values(&CellLoc::new(8, 8))
        );
    }

    #[test]
    fn get_box_values() {
        let board = get_board_a();
        assert_eq!(
            HashSet::from([8,4,9,7,6,5,1]),
            board.get_box_values(&CellLoc::new(2, 2))
        );
        assert_eq!(
            HashSet::from_iter(1..=9),
            board.get_box_values(&CellLoc::new(4, 4))
        );
        assert_eq!(
            HashSet::from([2,3,4,5,6,7,9]),
            board.get_box_values(&CellLoc::new(1,5))
        );
        assert_eq!(
            HashSet::from([1,4,8,9]),
            board.get_box_values(&CellLoc::new(8, 8))
        );
    }
}
