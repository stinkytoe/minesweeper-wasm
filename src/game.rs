//use std::{fmt, result};

use crate::{
    directions::DIRECTIONS,
    overlay::{Overlay, OverlayCell},
};

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Waiting,
    Playing,
    GameOver,
    GameWon,
}

pub struct Game {
    state: GameState,
    overlay: Option<Overlay>,
}

impl Game {
    pub fn begin_game(&mut self, rows: i32, cols: i32, minecount: i32) {
        if let Ok(new_overlay) = Overlay::new(rows, cols, minecount) {
            self.state = GameState::Playing;
            self.overlay = Some(new_overlay);
        }
    }

    pub fn dig(&mut self, row: i32, col: i32) {
        // We want to do a paint fill like effect whenever we encounter a '0' cell.
        // To do this, we fill a stack with all covered cells which surround a '0' cell, and
        // uncover those as well. If any of the uncovered cells are also '0' cells, then
        // we add the coordinate pair of the surrounding covered cells to the stack, and
        // repeat until exhausted.
        if let GameState::Playing = self.state {
            if let Some(overlay) = self.overlay.as_mut() {
                let mut stack = Vec::new();

                if overlay.is_covered(row, col) {
                    stack.push((row, col));
                }

                while let Some((row, col)) = stack.pop() {
                    // unwrap OK because all pairs are gated by a call to is_covered()
                    match overlay.dig(row, col).unwrap() {
                        OverlayCell::Exploded => {
                            self.state = GameState::GameOver;
                            overlay.reveal_mines();
                        }
                        OverlayCell::Uncovered(n) if n == 0 => {
                            for (d_row, d_col) in DIRECTIONS.iter() {
                                let row = row + d_row;
                                let col = col + d_col;

                                if overlay.is_covered(row, col) {
                                    stack.push((row, col));
                                }
                            }
                        }
                        _ => (),
                    }
                }

                //Now that the digging is done, check for victory and transition accordingly
                if overlay.is_victory_condition() {
                    self.state = GameState::GameWon;
                    overlay.flag_all_mines();
                }
            }
        }
    }

    pub fn get_cell(&self, row: i32, col: i32) -> Option<OverlayCell> {
        if let Some(overlay) = &self.overlay {
            overlay.get_cell(row, col)
        } else {
            None
        }
    }

    pub fn get_cols(&self) -> i32 {
        if let Some(overlay) = &self.overlay {
            overlay.get_cols()
        } else {
            0
        }
    }

    pub fn get_rows(&self) -> i32 {
        if let Some(overlay) = &self.overlay {
            overlay.get_rows()
        } else {
            0
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn new() -> Game {
        Game {
            state: GameState::Waiting,
            overlay: None,
        }
    }

    pub fn toggle_flag(&mut self, row: i32, col: i32) {
        if let GameState::Playing = self.state {
            if let Some(overlay) = self.overlay.as_mut() {
                if let Some(cell) = overlay.get_cell(row, col) {
                    match cell {
                        OverlayCell::Flagged => {
                            overlay.unflag(row, col);
                        }
                        OverlayCell::Covered => {
                            overlay.flag(row, col);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    pub fn get_remaining_mines(&self) -> i32 {
        if let Some(overlay) = &self.overlay {
            overlay.get_minecount() - overlay.get_flag_count()
        } else {
            0
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
