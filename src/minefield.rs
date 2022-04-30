#[derive(Clone, Copy, PartialEq)]
pub enum HiddenCell {
    Mine,
    Space,
}

pub struct Minefield {
    rows: i32,
    cols: i32,
    minecount: i32,
    hidden_layer: Vec<Vec<HiddenCell>>,
}

impl Minefield {
    pub fn is_valid_index(&self, row: i32, col: i32) -> bool {
        (col >= 0) && (row >= 0) && (col < self.cols) && (row < self.rows)
    }

    fn hidden_cell(&self, row: i32, col: i32) -> Option<HiddenCell> {
        match self.is_valid_index(row, col) {
            true => Some(self.hidden_layer[row as usize][col as usize]),
            false => None,
        }
    }

    pub fn is_mine(&self, row: i32, col: i32) -> bool {
        match self.hidden_cell(row, col) {
            Some(cell) => cell == HiddenCell::Mine,
            None => false,
        }
    }

    pub fn new(rows: i32, cols: i32, minecount: i32) -> Result<Minefield, String> {
        // Sanity checks
        if minecount > rows * cols {
            return Err(format!("More mines than cells in field to hold them! width: {} height: {} requested mines {}", rows, cols, minecount));
        }

        // Allocate heap space for storing the minefield
        let mut hidden_layer = vec![vec![HiddenCell::Space; cols as usize]; rows as usize];

        // populate the field with the given number of mines
        {
            use rand::prelude::*;
            let mut rng = rand::thread_rng();

            let mut mines_placed = 0;
            while mines_placed < minecount {
                let row_attempt: i32 = rng.gen_range(0..rows);
                let col_attempt: i32 = rng.gen_range(0..cols);

                if hidden_layer[row_attempt as usize][col_attempt as usize] == HiddenCell::Space {
                    hidden_layer[row_attempt as usize][col_attempt as usize] = HiddenCell::Mine;
                    mines_placed += 1;
                }
            }
        }

        Ok(Minefield {
            rows,
            cols,
            minecount,
            hidden_layer,
        })
    }

    pub fn get_rows(&self) -> i32 {
        self.rows
    }

    pub fn get_cols(&self) -> i32 {
        self.cols
    }

    pub fn get_minecount(&self) -> i32 {
        self.minecount
    }
}

#[test]
fn test_minefield_new() {
    let m1 = Minefield::new(10, 10, 100);
    assert!(m1.is_ok());
    let m2 = Minefield::new(10, 10, 101);
    assert!(m2.is_ok());
}
