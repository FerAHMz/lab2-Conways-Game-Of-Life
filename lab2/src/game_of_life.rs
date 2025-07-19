use crate::framebuffer::Framebuffer;

pub struct GameOfLife {
    pub cells: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        GameOfLife {
            cells: vec![vec![false; height]; width],
            width,
            height,
        }
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[x][y] = true;
        }
    }

    pub fn update(&mut self) {
        let mut new_cells = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let alive_neighbors = self.count_neighbors(x, y);
                let alive = self.cells[x][y];

                new_cells[x][y] = match (alive, alive_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        self.cells = new_cells;
    }

    pub fn render(&self, fb: &mut Framebuffer) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cells[x][y] {
                    fb.set_pixel(x as u32, y as u32);
                }
            }
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as isize + dx + self.width as isize) % self.width as isize;
                let ny = (y as isize + dy + self.height as isize) % self.height as isize;
                if self.cells[nx as usize][ny as usize] {
                    count += 1;
                }
            }
        }
        count
    }
}
