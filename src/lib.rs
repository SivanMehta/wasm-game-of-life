mod utils;

use wasm_bindgen::prelude::*;

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
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32, // 32 bits
    height: u32, // 32 bits
    cells: Vec<Cell> // a vector of cells
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn cells(&self) -> *const Cell {
        return self.cells.as_ptr();
    }

    // translate a given row and column into an index
    fn get_index(&self, row: u32, col: u32) -> usize {
        return (row * self.width + col) as usize
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

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0 ..self.width  {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let neighbors = self.live_neighbor_count(row, col);
                let next_cell = match(cell, neighbors) {
                    // any live cell with fewer than 2 neighbors dies (underpopulation)
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
        
                    // any live cell with two or three live neighbors lives on
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
        
                    // any live cell with more than three neighbors dies (overpopulation)
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
        
                    // any blank cell with 3 live neighbors lives (creation of life)
                    (Cell::Dead, 3) => Cell::Alive,
        
                    // otherwise
                    (otherwise, _) => otherwise,
                };
        
                next[index] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new(size: u32) -> Universe {
        let width = size;
        let height = size;

        let cells = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    return Cell::Alive
                } else {
                    return Cell::Dead
                }
            })
            .collect();
        
        let creation = Universe {
            width,
            height,
            cells,
        };

        return creation;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

use std::fmt;

// affords a to_string method
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // as_slice.chunks will essentially allow us to write this as a nessted loop
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}