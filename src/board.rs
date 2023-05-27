use std::str::FromStr;
use std::collections::HashSet;


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
            col,row,
            idx: row * 9 + col,
            base_size: 3,
            board_width: 9
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    cells: Vec<Option<u8>>,
    base_size: u8, // size of each 3x3 grid
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: vec![None; 81],
            base_size: 3,
        }
    }

    pub fn get(&self, loc: &CellLoc) -> Option<u8> {
        self.cells[loc.idx]
    }

    pub fn get_at(&self, c: usize, r: usize) -> Option<u8> {
        self.cells[CellLoc::new(c, r).idx]
    }

    pub fn set(&mut self, loc: &CellLoc, value: u8) -> Option<u8> {
        self.cells[loc.idx].replace(value)
    }

    pub fn set_at(&mut self, c: usize, r: usize, value: u8) -> Option<u8> {
        self.cells[CellLoc::new(c, r).idx].replace(value)
    }

    pub fn get_possible_moves(&self, loc: &CellLoc) -> HashSet<u8> {
        let c = self.get_col_values(&loc);
        let r = self.get_row_values(&loc);
        let b = self.get_box_values(&loc);

        (1..=9).filter(|v| !c.contains(v) && !r.contains(v) && !b.contains(v)).collect()
    }
    pub fn is_valid() -> bool {
        todo!()
    }
    pub fn is_done() -> bool {
        todo!()
    }

    pub fn get_col_values(&self, loc: &CellLoc) -> HashSet<u8> {
        (loc.col..self.cells.len()).step_by(loc.board_width).flat_map(|i| self.cells[i]).collect()
    }

    pub fn get_row_values(&self, loc: &CellLoc) -> HashSet<u8> {
        let start = loc.row * loc.board_width;
        self.cells[start..start+loc.board_width].iter().flat_map(|e| e.clone()).collect()
    }

    pub fn get_box_values(&self, loc: &CellLoc) -> HashSet<u8> {
        let mut set = HashSet::new();
        let c = loc.col / loc.base_size * loc.base_size;
        let r = loc.row / loc.base_size * loc.base_size;
        for x in c..c+loc.base_size {
            for y in r..r+loc.base_size {
                if let Some(value) = self.get_at(x,y){
                    set.insert(value);
                }
            }
        }
        set
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MalformedBoardError;

impl FromStr for Board {
    type Err = MalformedBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .chars()
            .map(|c| match c {
                '1'..='9' => c.to_digit(10).map(|x| x as u8),
                _ => None,
            })
            .collect();

        Ok(Board {
            cells,
            base_size: 3,
        })
    }
}