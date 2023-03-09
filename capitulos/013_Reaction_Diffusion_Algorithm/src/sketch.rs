use libfinal::color::pcolor4;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{constrain, floor};
use libfinal::image::{pcolor_to_tupla, pcolor_to_vector, PImage};

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 200;
pub const ALTO: u32 = 200;

// Aqui vendrá el pseudocódigo javascript
#[derive(Clone, Copy, Debug)]
struct Celda {
    a: f32,
    b: f32,
}

impl Celda {
    pub fn new(a: f32, b: f32) -> Celda {
        Celda { a, b }
    }
}

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    pimage: PImage,
    grid: Vec<Vec<Celda>>,
    next: Vec<Vec<Celda>>,
    da: f32,
    db: f32,

    feed: f32,
    k: f32,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        let mut grid: Vec<Vec<Celda>> =
            vec![vec![Celda::new(1.0, 0.0); ANCHO as usize]; ALTO as usize];
        let next: Vec<Vec<Celda>> = vec![vec![Celda::new(1.0, 0.0); ANCHO as usize]; ALTO as usize];

        for i in 100..110 {
            for j in 100..110 {
                grid[i][j].b = 1.0;
            }
        }

        engine.param.ancho = ANCHO as f32;
        engine.param.alto = ALTO as f32;

        Sketch {
            engine,
            pimage: PImage::default(),

            grid,
            next,
            da: 1.0,
            db: 0.5,

            feed: 0.055,
            k: 0.062,
        }
    }
    pub fn pre_load(&mut self) {
        full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self, canvas: &mut Canvas<Window>) {
        self.pimage.load_pixels(canvas);
    }

    pub fn update(&mut self) -> bool {
        for x in 1..(ANCHO - 1) as usize {
            for y in 1..(ALTO - 1) as usize {
                let a = self.grid[x][y].a;
                let b = self.grid[x][y].b;

                self.next[x][y].a =
                    a + self.da * self.laplace_a(x, y) - a * b * b + self.feed * (1.0 - a);

                self.next[x][y].b =
                    b + self.db * self.laplace_b(x, y) + a * b * b - (self.k + self.feed) * b;

                self.next[x][y].a = constrain(self.next[x][y].a, 0.0, 1.0);
                self.next[x][y].b = constrain(self.next[x][y].b, 0.0, 1.0);
            }
        }

        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        for x in 0..ANCHO as usize {
            for y in 0..ALTO as usize {
                //let pix = x + y * ANCHO as usize;

                let a = self.next[x][y].a;
                let b = self.next[x][y].b;
                let mut c = floor((a - b) * 255.0);
                c = constrain(c, 0.0, 255.0);

                //let color = Color::new(c as u8, c as u8, c as u8, 255);
                let color = pcolor4(c as u8, c as u8, c as u8, 255);

                //self.engine.param.pixels[pix] = color;
                self.pimage.image[x][y] = pcolor_to_tupla(color);
            }
        }
        // Actualiza la ventana de visualización con los datos del arreglo en Self de image.
        self.pimage.update_pixels(canvas);

        self.swap();
    }

    pub fn swap(&mut self) {
        let mut temp = vec![vec![Celda::new(0.0, 0.0); ANCHO as usize]; ALTO as usize];
        for i in 0..self.grid.len() {
            for j in 0..self.next.len() {
                temp[i][j] = self.grid[i][j].clone();
                self.grid[i][j] = self.next[i][j].clone();
                self.next[i][j] = temp[i][j].clone();
            }
        }
    }

    pub fn laplace_a(&mut self, x: usize, y: usize) -> f32 {
        //pub fn laplace_a(&mut self) -> f32 {
        let mut suma: f32 = 0.0;

        suma += self.grid[x][y].a * -1.0;
        suma += self.grid[x - 1][y].a * 0.2;
        suma += self.grid[x + 1][y].a * 0.2;
        suma += self.grid[x][y + 1].a * 0.2;
        suma += self.grid[x][y - 1].a * 0.2;
        suma += self.grid[x - 1][y - 1].a * 0.05;
        suma += self.grid[x + 1][y - 1].a * 0.05;
        suma += self.grid[x + 1][y + 1].a * 0.05;
        suma += self.grid[x - 1][y + 1].a * 0.05;

        suma
    }
    pub fn laplace_b(&mut self, x: usize, y: usize) -> f32 {
        //pub fn laplace_b(&mut self) -> f32 {
        let mut sumb: f32 = 0.0;

        sumb += self.grid[x][y].b * -1.0;
        sumb += self.grid[x - 1][y].b * 0.2;
        sumb += self.grid[x + 1][y].b * 0.2;
        sumb += self.grid[x][y + 1].b * 0.2;
        sumb += self.grid[x][y - 1].b * 0.2;
        sumb += self.grid[x - 1][y - 1].b * 0.05;
        sumb += self.grid[x + 1][y - 1].b * 0.05;
        sumb += self.grid[x + 1][y + 1].b * 0.05;
        sumb += self.grid[x - 1][y + 1].b * 0.05;

        sumb
    }
}
