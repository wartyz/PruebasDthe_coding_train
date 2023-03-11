use libfinal::color::{color_mode, background, fill3, no_stroke};
use libfinal::parametros::ColorMode;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::shape::ellipse;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    n: f32,
    c: f32,
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
        Sketch {
            engine,
            n: 0.0,
            c: 4.0,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self, canvas: &mut Canvas<Window>) {
        color_mode(ColorMode::HSB, &mut self.engine.param);
        background(canvas, &mut self.engine, 0);
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let a = self.n * 137.5;

        let r = self.c * self.n.sqrt();

        let x = r * a.cos() + self.engine.param.ancho / 2.0;
        let y = r * a.sin() + self.engine.param.alto / 2.0;

        //fill1(255, &mut self.engine.param);
        fill3((a - r) % 255.0, 255.0, 255.0, &mut self.engine.param);
        no_stroke(&mut self.engine.param);
        ellipse(canvas, &mut self.engine.param, x, y, 4.0, 4.0);

        self.n += 1.0;
    }
}
