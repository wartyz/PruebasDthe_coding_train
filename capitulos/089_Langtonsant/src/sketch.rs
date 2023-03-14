use libfinal::color::{background, pcolor1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::image::PImage;

// Ancho y alto de la pantalla
pub const ANCHO: usize = 1000;
pub const ALTO: usize = 1000;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del sketch
    grid: Vec<Vec<i32>>,
    x: i32,
    y: i32,
    dir: i32,
    ant: PImage,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

const ANTUP: i32 = 0;
const ANTRIGHT: i32 = 1;
const ANTDOWN: i32 = 2;
const ANTLEFT: i32 = 3;

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        engine.param.ancho = ANCHO as f32;
        engine.param.alto = ALTO as f32;
        Sketch {
            engine,
            grid: vec![vec![0; ALTO as usize]; ANCHO as usize],
            x: 0,
            y: 0,
            dir: 0,
            ant: PImage::create_image(ANCHO as u32, ALTO as u32),
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self, canvas: &mut Canvas<Window>) {
        self.ant.load_pixels(canvas);

        for alt in 0..(self.ant.image_height - 500) as usize {
            for anch in 0..(self.ant.image_width - 500) as usize {
                self.ant.image[anch][alt] = (255, 255, 255, 255);
            }
        }

        self.ant.update_pixels(canvas);
        self.x = ANCHO as i32 / 2;
        self.y = ALTO as i32 / 2;
        self.dir = ANTUP;
    }

    pub fn turn_right(&mut self) {
        self.dir += 1;
        if self.dir > ANTLEFT {
            self.dir = ANTUP;
        }
    }

    pub fn turn_left(&mut self) {
        self.dir -= 1;
        if self.dir < ANTUP {
            self.dir = ANTLEFT;
        }
    }

    pub fn move_forward(&mut self) {
        if self.dir == ANTUP {
            self.y -= 1;
        } else if self.dir == ANTRIGHT {
            self.x += 1;
        } else if self.dir == ANTDOWN {
            self.y += 1;
        } else if self.dir == ANTLEFT {
            self.x -= 1;
        }

        if self.x > ANCHO as i32 - 1 {
            self.x = 0;
        } else if self.x < 0 {
            self.x = ANCHO as i32 - 1;
        }
        if self.y > ALTO as i32 - 1 {
            self.y = 0;
        } else if self.y < 0 {
            self.y = ALTO as i32 - 1;
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
        background(canvas, &mut self.engine, 255);
        self.ant.load_pixels(canvas);
        for _ in 0..100000 {
            let x = self.x as usize;
            let y = self.y as usize;

            let state = self.grid[x][y];
            if state == 0 {
                self.turn_right();
                self.grid[x][y] = 1;
            } else if state == 1 {
                self.turn_left();
                self.grid[x][y] = 0;
            }

            let mut col = pcolor1(255);
            if self.grid[x][y] == 1 {
                col = pcolor1(0);
            }

            self.ant.image[x][y] = (col.r, col.g, col.b, col.a);
            self.move_forward();
        }
        self.ant.update_pixels(canvas);
    }
}
