use libfinal::color::{background, fill1, stroke1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{floor, random, random_usize};
use libfinal::shape::rect;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1800;
pub const ALTO: i32 = 1200;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del sketch
    grid: Vec<Vec<usize>>,
    cols: usize,
    rows: usize,
    resolution: i32,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        let resolution = 10;
        let cols = (ANCHO / resolution) as usize;
        let rows = (ALTO / resolution) as usize;

        let mut grid = vec![vec![0usize; rows]; cols];

        for i in 0..cols {
            for j in 0..rows {
                grid[i][j] = floor(random(2.)) as usize;
            }
        }

        engine.param.ancho = ANCHO as f32;
        engine.param.alto = ALTO as f32;
        Sketch {
            engine,
            grid,
            cols,
            rows,
            resolution,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        for i in 0..self.cols {
            for j in 0..self.rows {
                self.grid[i][j] = random_usize(2);
            }
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        background(canvas, &mut self.engine, 51);

        for i in 0..self.cols {
            for j in 0..self.rows {
                let x = (i * self.resolution as usize) as f32;
                let y = (j * self.resolution as usize) as f32;
                if self.grid[i][j] == 1 {
                    fill1(255., &mut self.engine.param);
                    stroke1(0., &mut self.engine.param);
                    rect(
                        canvas,
                        &mut self.engine.param,
                        x,
                        y,
                        (self.resolution - 1) as f32,
                        (self.resolution - 1) as f32,
                    );
                }
            }
        }

        let mut next = vec![vec![0usize; self.rows as usize]; self.cols as usize];

        // Compute next based on grid
        for i in 0..self.cols {
            for j in 0..self.rows {
                let state = self.grid[i][j];

                // Count live neighbors
                //sum := 0

                let neighbors = &self.count_neighbors(i, j);

                if state == 0 && *neighbors == 3 {
                    next[i][j] = 1;
                } else if state == 1 && (*neighbors < 2 || *neighbors > 3) {
                    next[i][j] = 0;
                } else {
                    next[i][j] = state;
                }
            }
        }
        self.grid = next
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;
        for i in -1..2i32 {
            for j in -1..2i32 {
                let col = ((x as i32 + i + self.cols as i32) % self.cols as i32) as usize;
                let row = ((y as i32 + j + self.rows as i32) % self.rows as i32) as usize;

                sum += self.grid[col][row];
            }
        }
        sum -= self.grid[x][y];
        sum
    }
}
