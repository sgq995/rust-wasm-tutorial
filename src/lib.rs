mod utils;

extern crate js_sys;

extern crate fixedbitset;

use wasm_bindgen::prelude::*;

use std::fmt;

use fixedbitset::FixedBitSet;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_clamp_index(&self, row: u32, column: u32) -> usize {
        let clamp_row = row % self.height;
        let clamp_col = column % self.width;
        self.get_index(clamp_row, clamp_col)
    }
    
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn get_cells(&self) -> &[u32] {
        self.cells.as_slice()
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                // );

                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };
                // log!("  it becomes {:?}", next_cell);
                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn random(&mut self) {
        let size = self.cells.len();
        for i in 0..size {
            self.cells.set(i, js_sys::Math::random() < 0.5);
        }
    }

    pub fn killall(&mut self) {
        self.cells.clear();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        // self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
        // self.cells.grow((width * self.height) as usize);
        let size = (width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        // self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
        // self.cells.grow((self.width * height) as usize);
        let size = (self.width * height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn glider(&mut self, row: u32, column: u32) {
        let model: [[i8; 3]; 3] = [
            [0, 1, 0],
            [0, 0, 1],
            [1, 1, 1],
        ];

        // for line in &model {
        //     for n in line {
        //         if *n == 0 {

        //         } else {

        //         }
        //     }
        // }

        let mut idx: usize;
        let mut current_row: u32;
        let mut current_col: u32;

        //
        current_row = row - 1 + self.height - 1;

        current_col = column - 1 + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, false);

        current_col = column + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, true);

        current_col = column + 1 + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, false);

        //
        current_row = row + self.height - 1;

        current_col = column - 1 + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, false);

        current_col = column + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, false);

        current_col = column + 1 + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, true);

        //
        current_row = row + 1 + self.height - 1;

        current_col = column - 1 + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, true);

        current_col = column + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, true);

        current_col = column + 1 + self.width - 1;
        idx = self.get_clamp_index(current_row, current_col);
        self.cells.set(idx, true);
    }

    pub fn pulsar(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);

        // TODO:
        let model: [[i8; 13]; 13] = [
            [0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            [0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            [1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
        ];
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell != 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
