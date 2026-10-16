use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width,
            height,
            cells: vec![0; (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells_ptr(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        if row >= self.height || col >= self.width {
            return;
        }

        let index = (row * self.width + col) as usize;
        self.cells[index] ^= 1;
    }

    pub fn set_cell(&mut self, row: u32, col: u32, alive: bool) {
        if row >= self.height || col >= self.width {
            return;
        }

        let index = (row * self.width + col) as usize;
        self.cells[index] = u8::from(alive);
    }

    pub fn clear(&mut self) {
        self.cells.fill(0);
    }

    pub fn randomize(&mut self) {
        for cell in &mut self.cells {
            *cell = u8::from(js_sys::Math::random() < 0.22);
        }
    }

    pub fn reset_glider(&mut self) {
        self.clear();

        let start_x = self.width / 2;
        let start_y = self.height / 2;

        let pattern = [
            (1, 0),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];

        for (dx, dy) in pattern {
            self.set_cell(start_y + dy, start_x + dx, true);
        }
    }

    pub fn tick(&mut self) {
        let mut next = vec![0; self.cells.len()];

        for row in 0..self.height {
            for col in 0..self.width {
                let index = (row * self.width + col) as usize;
                let neighbors = self.live_neighbor_count(row, col);
                let alive = self.cells[index] == 1;

                next[index] = u8::from(
                    neighbors == 3 || (alive && neighbors == 2)
                );
            }
        }

        self.cells = next;
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for dy in -1_i32..=1 {
            for dx in -1_i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let x = col as i32 + dx;
                let y = row as i32 + dy;

                if x < 0
                    || y < 0
                    || x >= self.width as i32
                    || y >= self.height as i32
                {
                    continue;
                }

                let index = (y as u32 * self.width + x as u32) as usize;

                if self.cells[index] == 1 {
                    count += 1;
                }
            }
        }

        count
    }
}
