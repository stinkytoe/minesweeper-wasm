//extern crate cfg_if;
extern crate wasm_bindgen;

//mod utils;

mod directions;
mod game;
mod minefield;
mod overlay;

use game::GameState;
use overlay::OverlayCell;
use wasm_bindgen::prelude::*;

enum Marked {
    None,
    One((i32, i32)),
    Multi(Vec<(i32, i32)>),
}

#[wasm_bindgen]
pub struct WasmGame {
    game: game::Game,
    marked: Marked,
}

impl Default for WasmGame {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WasmGame {
    pub fn new() -> WasmGame {
        WasmGame {
            game: game::Game::new(),
            marked: Marked::None,
        }
    }

    pub fn mark(&mut self, row: i32, col: i32) {
        if let GameState::Playing = self.game.get_state() {
            self.marked = Marked::One((row, col));
        }
    }

    pub fn mark_block(&mut self, row: i32, col: i32) {
        if let GameState::Playing = self.game.get_state() {
            let mut v = Vec::with_capacity(9);

            for (d_row, d_col) in directions::DIRECTIONS.iter() {
                v.push((row + d_row, col + d_col));
            }

            v.push((row, col));

            self.marked = Marked::Multi(v);
        }
    }

    pub fn stub(&mut self) {
        self.game.begin_game(30, 16, 20);
    }

    pub fn get_rows(&self) -> i32 {
        self.game.get_rows()
    }

    pub fn get_cols(&self) -> i32 {
        self.game.get_cols()
    }

    pub fn get_cell_code(&self, row: i32, col: i32) -> u8 {
        if let Some(cell) = self.game.get_cell(row, col) {
            match cell {
                overlay::OverlayCell::Covered => match &self.marked {
                    Marked::None => 12,
                    Marked::One((x_row, x_col)) => {
                        if (row, col) == (*x_row, *x_col) {
                            0
                        } else {
                            12
                        }
                    }
                    Marked::Multi(v) => {
                        if v.contains(&(row, col)) {
                            0
                        } else {
                            12
                        }
                    }
                },
                overlay::OverlayCell::Uncovered(n) if n == 0 => 0,
                overlay::OverlayCell::Uncovered(n) => n + 3,
                overlay::OverlayCell::Flagged => 13,
                overlay::OverlayCell::Exploded => 3,
                overlay::OverlayCell::Mine => 1,
                overlay::OverlayCell::BadFlag => 2,
            }
        } else {
            0
        }
    }

    pub fn dig_marked(&mut self) {
        match &self.marked {
            Marked::None => (),
            Marked::One((row, col)) => {
                self.game.dig(*row, *col);
            }
            Marked::Multi(v) => {
                let (row, col) = &v[v.len() - 1..][0];
                if let Some(OverlayCell::Uncovered(surrounding_mines)) =
                    self.game.get_cell(*row, *col)
                {
                    let mut surrounding_flags = 0;

                    for (row, col) in &v[..v.len() - 1] {
                        if let Some(OverlayCell::Flagged) = self.game.get_cell(*row, *col) {
                            surrounding_flags += 1;
                        }
                    }

                    if surrounding_flags == surrounding_mines {
                        for (row, col) in &v[..v.len() - 1] {
                            self.game.dig(*row, *col);
                        }
                    }
                }
            }
        }
        self.marked = Marked::None;
    }

    pub fn toggle_flag(&mut self, row: i32, col: i32) {
        self.game.toggle_flag(row, col);
    }

    pub fn get_game_state(&self) -> String {
        match self.game.get_state() {
            game::GameState::Waiting => todo!(),
            game::GameState::Playing => todo!(),
            game::GameState::GameOver => todo!(),
            game::GameState::GameWon => todo!(),
        }
    }
}
