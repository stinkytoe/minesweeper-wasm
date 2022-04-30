use crate::{directions::DIRECTIONS, minefield::Minefield};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayCell {
    Covered,
    Uncovered(u8),
    Flagged,
    Exploded,
    Mine,
    BadFlag,
}

pub struct Overlay {
    minefield: Minefield,
    overlay_layer: Vec<Vec<OverlayCell>>,
}
impl Overlay {
    pub fn new(rows: i32, cols: i32, minecount: i32) -> Result<Overlay, String> {
        // pass to Minefield::new, which both determines if this is a valid construction,
        // and makes us our minefield
        let minefield = Minefield::new(rows, cols, minecount);

        // if the minefield constructor fails, we also fail
        if let Err(e) = minefield {
            return Err(format!("Couldn't create minefield!: {}", e));
        };

        // Otherwise, we can unwrap it
        let minefield = minefield.unwrap();

        // Generate the overlay struct, and allocate the heap for the overlay_layer
        Ok(Overlay {
            minefield,
            overlay_layer: vec![vec![OverlayCell::Covered; cols as usize]; rows as usize],
        })
    }

    pub fn get_cell(&self, row: i32, col: i32) -> Option<OverlayCell> {
        match self.minefield.is_valid_index(row, col) {
            true => Some(self.overlay_layer[row as usize][col as usize]),
            false => None,
        }
    }

    pub fn is_covered(&self, row: i32, col: i32) -> bool {
        self.minefield.is_valid_index(row, col)
            && self.overlay_layer[row as usize][col as usize] == OverlayCell::Covered
    }

    fn get_cell_mut(&mut self, row: i32, col: i32) -> Option<&mut OverlayCell> {
        match self.minefield.is_valid_index(row, col) {
            true => Some(&mut self.overlay_layer[row as usize][col as usize]),
            false => None,
        }
    }

    // We dig.
    // If it's not a valid click, we return None
    // If this is anything but a covered spot, we simply return what is there.
    // If it's covered, we uncover.
    //   If it's a mine, we explode it.
    //   If it's not a mine, then we change to an uncovered,
    //    calculate the surrounding mines and store in the cell.
    pub fn dig(&mut self, row: i32, col: i32) -> Option<OverlayCell> {
        let cell = self.get_cell(row, col);

        cell?;

        let cell = cell.unwrap();

        if cell != OverlayCell::Covered {
            return Some(cell);
        }

        if self.minefield.is_mine(row, col) {
            // It's safe to .unwrap() here because of the checks after
            // the above call to .get_cell()
            let cell_mut = self.get_cell_mut(row, col).unwrap();
            *cell_mut = OverlayCell::Exploded;
            Some(*cell_mut)
        } else {
            let mut count = 0;
            for (d_row, d_col) in DIRECTIONS {
                if self.minefield.is_mine(row + d_row, col + d_col) {
                    count += 1;
                }
            }

            // It's safe to .unwrap() here because of the checks after
            // the above call to .get_cell()
            let cell_mut = self.get_cell_mut(row, col).unwrap();
            *cell_mut = OverlayCell::Uncovered(count);
            Some(*cell_mut)
        }
    }

    // if it's a covered cell, change to OverlayCell::Flagged
    // all other cases, do nothing
    pub fn flag(&mut self, row: i32, col: i32) {
        if let Some(cell) = self.get_cell_mut(row, col) {
            if *cell == OverlayCell::Covered {
                *cell = OverlayCell::Flagged;
            }
        }
    }

    pub fn unflag(&mut self, row: i32, col: i32) {
        if let Some(cell) = self.get_cell_mut(row, col) {
            if *cell == OverlayCell::Flagged {
                *cell = OverlayCell::Covered;
            }
        }
    }

    pub fn get_rows(&self) -> i32 {
        self.minefield.get_rows()
    }

    pub fn get_cols(&self) -> i32 {
        self.minefield.get_cols()
    }

    pub fn reveal_mines(&mut self) {
        for row in 0..self.get_rows() {
            for col in 0..self.get_cols() {
                if let Some(cell) = self.get_cell(row, col) {
                    match cell {
                        OverlayCell::Covered if self.minefield.is_mine(row, col) => {
                            if let Some(cell_mut) = self.get_cell_mut(row, col) {
                                *cell_mut = OverlayCell::Mine;
                            }
                        }
                        OverlayCell::Flagged => if !self.minefield.is_mine(row, col) {
                            if let Some(cell_mut) = self.get_cell_mut(row, col) {
                                *cell_mut = OverlayCell::BadFlag;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
