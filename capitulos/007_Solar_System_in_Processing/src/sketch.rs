use std::f32::consts::PI;
use libfinal::color::background;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::random;
use libfinal::transform::translate;

use crate::planet::Planet;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 600;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    sun: Planet,
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
            sun: Planet::new(50.0, 0.0, 0.0, random(PI * 2.)),
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        self.sun.spawn_moons(5, 1.0);
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
        translate(
            ANCHO as f32 / 2.0,
            ALTO as f32 / 2.0,
            &mut self.engine.param,
        );
        self.sun.show(&mut self.engine, canvas);
        self.sun.orbit();
    }
}
