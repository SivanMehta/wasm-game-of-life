mod utils;
mod fps;

use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32, // 32 bits
    height: u32, // 32 bits
    cells: FixedBitSet // a vector of cells
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn cells(&self) -> *const u32 {
        return self.cells.as_slice().as_ptr();
    }

    // translate a given row and column into an index
    fn get_index(&self, row: u32, col: u32) -> usize {
        return (row * self.width + col) as usize;
    }

    #[allow(unused_comparisons)]
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [row - 1, row, row + 1].iter().cloned() {
            for delta_col in [column - 1, column, column + 1].iter().cloned() {
                // if we are looking at ourselves
                if delta_row == row && delta_col == column {
                    continue;
                }
                // if we are out of bounds (eg: the edges)
                let out_of_bounds =
                    delta_col >= self.width ||
                    delta_col < 0 ||
                    delta_row >= self.height ||
                    delta_row < 0;

                if out_of_bounds {
                    continue;
                }

                let idx = self.get_index(delta_row, delta_col);
                count += self.cells.contains(idx) as u8;
            }
        }
        return count;
    }

    pub fn tick(&mut self) -> Vec<usize> {
        let mut next = self.cells.clone();
        let capacity = self.capacity();
        let mut same_cells = Vec::with_capacity(capacity);

        for row in 0..self.height {
            for col in 0 ..self.width  {
                let index = self.get_index(row, col);
                let cell = self.cells.contains(index);
                let neighbors = self.live_neighbor_count(row, col);
                let next_cell = match(cell, neighbors) {
                    // any live cell with fewer than 2 neighbors dies (underpopulation)
                    (true, x) if x < 2 => false,
        
                    // any live cell with two or three live neighbors lives on
                    (true, 2) | (true, 3) => true,
        
                    // any live cell with more than three neighbors dies (overpopulation)
                    (true, x) if x > 3 => false,
        
                    // any blank cell with 3 live neighbors lives (creation of life)
                    (false, 3) => true,
        
                    // otherwise
                    (otherwise, _) => otherwise,
                };
        
                next.set(index, next_cell);
                if next_cell == cell {
                    same_cells.insert(index, 1);
                } else {
                    same_cells.insert(index, 0);
                }
            }
        }

        self.cells = next;
        return same_cells;
    }

    #[wasm_bindgen(constructor)]
    pub fn new(size: u32) -> Universe {
        let width = size;
        let height = size;

        let capacity = (size * size) as usize;
        let mut cells = FixedBitSet::with_capacity(capacity);

        for bit in 0..capacity {
            let value = js_sys::Math::random() < 0.5;
            cells.set(bit, value)
        }
        
        let creation = Universe {
            width,
            height,
            cells,
        };

        return creation;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let index = self.get_index(row, col);
        println!("{}, {}, {}", row, col, index);
        self.cells.toggle(index);
    }

    fn capacity(&self) -> usize {
        return (self.width * self.width) as usize;
    }

    fn generate_cells(&mut self, generator: fn(usize) -> bool) {
        let capacity = self.capacity();
        let mut new_cells = FixedBitSet::with_capacity(capacity);
        for bit in 0..capacity {
            let value = generator(bit);
            new_cells.set(bit, value)
        }

        self.cells = new_cells;
    }

    pub fn clear(&mut self) {
        fn generator(_: usize) -> bool { return false; }
        self.generate_cells(generator)
    }

    pub fn reset(&mut self) {
        fn generator(_: usize) -> bool { return js_sys::Math::random() < 0.5 }
        self.generate_cells(generator);
    }
}