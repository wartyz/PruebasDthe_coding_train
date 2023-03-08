use libfinal::color::background;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::mapa;
use libfinal::transform::translate;
use crate::star::Star;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 600;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    // Variables globales del scketch
    stars: Vec<Star>,
    max_stars: i32,
    speed: f32,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,
            stars: vec![],
            max_stars: 800,
            speed: 0.0,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        for _ in 0..self.max_stars {
            self.stars.push(Star::new(&self.engine.param));
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
        background(canvas, &mut self.engine, 0);

        self.speed = mapa(
            self.engine.param.mouse_posicion.x,
            0.0,
            self.engine.param.ancho,
            0.0,
            50.0,
        );

        translate(
            self.engine.param.ancho / 2.0,
            self.engine.param.alto / 2.0,
            &mut self.engine.param,
        );

        for i in 0..self.stars.len() {
            self.stars[i].update(&self.engine, self.speed);
            self.stars[i].show(&mut self.engine, canvas);
        }
    }

    pub fn key_pressed(&mut self) {
        //        if self.engine.param.key == CodigosTecla::A {
        //            self.left.mover(-10.0);
        //        } else if self.engine.param.key == CodigosTecla::Z {
        //            self.left.mover(10.0)
        //        } else if self.engine.param.key == CodigosTecla::J {
        //            self.right.mover(-10.0)
        //        } else if self.engine.param.key == CodigosTecla::M {
        //            self.right.mover(10.0)
        //        }
    }

    pub fn key_released(&mut self) {
        //        if self.engine.param.keyr == CodigosTecla::A {
        //            self.left.mover(0.0);
        //        } else if self.engine.param.keyr == CodigosTecla::Z {
        //            self.left.mover(0.0)
        //        } else if self.engine.param.keyr == CodigosTecla::J {
        //            self.right.mover(0.0)
        //        } else if self.engine.param.keyr == CodigosTecla::M {
        //            self.right.mover(0.0)
        //        }
    }
}
