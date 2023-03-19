use libfinal::color::{background, pcolor1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::image::PImage;
use libfinal::parametros::CodigosRaton::Izquierdo;
use libfinal::parametros::{CodigosRaton, Parametros};

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del sketch
    cols: u32,
    rows: u32,
    current: Vec<Vec<f32>>,
    previous: Vec<Vec<f32>>,

    dampening: f32,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        engine.param.ancho = ANCHO as f32;
        engine.param.alto = ALTO as f32;

        let cols = ANCHO;
        let rows = ALTO;
        Sketch {
            engine,
            cols,
            rows,
            previous: vec![vec![0.; rows as usize]; cols as usize],
            current: vec![vec![0.; rows as usize]; cols as usize],

            dampening: 0.99,

        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {}

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        background(canvas, &mut self.engine, 0);
        let mut pimagen = PImage::create_image(ANCHO, ALTO);
        pimagen.load_pixels(canvas);
        for i in 1..(self.cols - 1) as usize {
            for j in 1..(self.rows - 1) as usize {
                self.current[i][j] = (
                    self.previous[i - 1][j] +
                        self.previous[i + 1][j] +
                        self.previous[i][j - 1] +
                        self.previous[i][j + 1]) / 2. - self.current[i][j];
                self.current[i][j] = self.current[i][j] * self.dampening;

                let cc = self.current[i][j] as u8;
                pimagen.image[i][j] = (cc, cc, cc, 255);
            }
        }
        pimagen.update_pixels(canvas);

        let temp = self.previous.clone();
        self.previous = self.current.clone();
        self.current = temp;
    }

    pub fn mouse_dragged(&mut self) {
        self.previous[self.engine.param.mouse_posicion.x as usize]
            [self.engine.param.mouse_posicion.y as usize] = 500.;
    }
}
