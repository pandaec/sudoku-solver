use std::collections::BTreeSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct CellLoc {
    col: usize,
    row: usize,
    idx: usize,
    base_size: usize,
    board_width: usize,
}

impl CellLoc {
    pub fn new(col: usize, row: usize) -> Self {
        Self {
            col,
            row,
            idx: row * 9 + col,
            base_size: 3,
            board_width: 9,
        }
    }

    pub fn idx(idx: usize) -> Self {
        Self {
            col: idx % 9,
            row: idx / 9,
            idx,
            base_size: 3,
            board_width: 9,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cell {
    pub idx: usize,
    pub value: Option<u8>,
    pub is_variable: bool,
}

impl Cell {
    pub fn new(idx: usize, value: Option<u8>, is_variable: bool) -> Self {
        Self {
            idx,
            value,
            is_variable,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    pub cells: Vec<Cell>, //temp
    base_size: u8,        // size of each 3x3 grid
}

impl Board {
    pub fn get(&self, loc: &CellLoc) -> Option<u8> {
        self.cells[loc.idx].value
    }

    pub fn get_at(&self, c: usize, r: usize) -> Option<u8> {
        self.cells[CellLoc::new(c, r).idx].value
    }

    pub fn set(&mut self, loc: &CellLoc, v: u8) -> Option<u8> {
        self.cells[loc.idx].value.replace(v)
    }

    // TODO use set and allow Option instead
    pub fn set_none(&mut self, loc: &CellLoc) {
        self.cells[loc.idx].value = None;
    }
    // TODO unify setter
    pub fn set_none_range(&mut self, start: usize, end: usize) {
        self.cells[start..=end].iter_mut().for_each(|cell| {
            if cell.is_variable {
                cell.value = None;
            }
        })
    }

    pub fn set_at(&mut self, c: usize, r: usize, v: u8) -> Option<u8> {
        self.cells[CellLoc::new(c, r).idx].value.replace(v)
    }

    pub fn get_possible_moves(&self, loc: &CellLoc) -> BTreeSet<u8> {
        let c = self.get_col_values(&loc);
        let r = self.get_row_values(&loc);
        let b = self.get_box_values(&loc);

        (1..=9)
            .filter(|v| !c.contains(v) && !r.contains(v) && !b.contains(v))
            .collect()
    }
    pub fn is_valid() -> bool {
        todo!()
    }
    pub fn is_done() -> bool {
        todo!()
    }

    pub fn get_col_values(&self, loc: &CellLoc) -> BTreeSet<u8> {
        (loc.col..self.cells.len())
            .step_by(loc.board_width)
            .flat_map(|i| self.cells[i].value)
            .collect()
    }

    pub fn get_row_values(&self, loc: &CellLoc) -> BTreeSet<u8> {
        let start = loc.row * loc.board_width;
        self.cells[start..start + loc.board_width]
            .iter()
            .flat_map(|e| e.value.clone())
            .collect()
    }

    pub fn get_box_values(&self, loc: &CellLoc) -> BTreeSet<u8> {
        let mut set = BTreeSet::new();
        let c = loc.col / loc.base_size * loc.base_size;
        let r = loc.row / loc.base_size * loc.base_size;
        for x in c..c + loc.base_size {
            for y in r..r + loc.base_size {
                if let Some(value) = self.get_at(x, y) {
                    set.insert(value);
                }
            }
        }
        set
    }

    pub fn find_first_variable(&self, idx: usize) -> Option<&Cell> {
        self.cells[idx..].iter().find(|cell| cell.is_variable)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MalformedBoardError;

impl FromStr for Board {
    type Err = MalformedBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '1'..='9' => Cell::new(i, c.to_digit(10).map(|x| x as u8), false),
                _ => Cell::new(i, None, true),
            })
            .collect();

        Ok(Board {
            cells,
            base_size: 3,
        })
    }
}
