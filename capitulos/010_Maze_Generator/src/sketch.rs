use libfinal::color::background;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::floor;
use crate::cell::Cell;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 400;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    cols: i32,
    rows: i32,
    w: f32,
    grid: Vec<Cell>,
    current: Option<usize>,
    stack: Vec<Option<usize>>,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);
        let w = 10.0;

        engine.param.ancho = ANCHO as f32;
        engine.param.alto = ALTO as f32;
        Sketch {
            engine,
            cols: floor(ANCHO as f32 / w) as i32,
            rows: floor(ANCHO as f32 / w) as i32,
            w,
            grid: vec![],
            current: None,
            stack: vec![],
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        for j in 0..self.rows {
            for i in 0..self.cols {
                let cell = Cell::new(i as i32, j as i32);
                self.grid.push(cell);
            }
        }
        self.current = Some(0);
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

        for g in &mut self.grid {
            g.show(self.w, &mut self.engine, canvas);
        }

        let mut next = None;
        if self.current.is_some() {
            self.grid[self.current.unwrap()].visited = true;
            self.grid[self.current.unwrap()].highlight(self.w, &mut self.engine, canvas);

            //STEP 1
            let indice_next = self.current.unwrap();
            let mut n = self.grid[indice_next];
            next = n.check_neighbors(self.cols, self.rows, &mut self.grid);
        }

        //println!("en draw() 1  next = {:?}", next);
        if next.is_some() {
            self.grid[next.unwrap()].visited = true;

            // STEP 2 Hace PUSH en el stack
            self.stack.push(self.current);
            // STEP 3
            self.remove_walls(self.current, next);

            self.current = next;
        } else {
            if self.stack.len() > 0 {
                // hace POP del stack
                self.current = self.stack.pop().unwrap();
            }
        }
    }

    pub fn remove_walls(&mut self, a: Option<usize>, b: Option<usize>) {
        let x = self.grid[a.unwrap()].i - self.grid[b.unwrap()].i;
        if x == 1 {
            self.grid[a.unwrap()].walls[3] = false;
            self.grid[b.unwrap()].walls[1] = false;
        } else if x == -1 {
            self.grid[a.unwrap()].walls[1] = false;
            self.grid[b.unwrap()].walls[3] = false;
        }

        let y = self.grid[a.unwrap()].j - self.grid[b.unwrap()].j;

        if y == 1 {
            self.grid[a.unwrap()].walls[0] = false;
            self.grid[b.unwrap()].walls[2] = false;
        } else if y == -1 {
            self.grid[a.unwrap()].walls[2] = false;
            self.grid[b.unwrap()].walls[0] = false;
        }
    }
}
