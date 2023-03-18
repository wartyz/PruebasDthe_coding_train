use std::f32::consts::PI;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::rendering;
use libfinal::rendering::{create_graphics, PGraphics};

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 900;
pub const ALTO: u32 = 600;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del sketch
    r1: f32,

    r2: f32,
    m1: f32,
    m2: f32,
    a1: f32,
    a2: f32,
    a1_v: f32,
    a2_v: f32,
    g: f32,

    px2: f32,
    py2: f32,
    cx: f32,
    cy: f32,

    canvas: PGraphics,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        let r1 = 200.; // length of first pendulum
        let r2 = 200.; // length of second pendulum
        let m1 = 40.; //  mass of first pendulum excluding weight of string
        let m2 = 40.; // mass of second pendulum excluding weight of string
        let a1 = PI / 2.; // angle formed by first pendulum and normal - angle1
        let a2 = PI / 2.; //angle formed by second pendulum and normal - angle2
        let a1_v = 0.; //angular velocity of pendulum1
        let a2_v = 0.; //angular velocity of pendulum2
        let g = 1.; //gravitational constant (realistic value not considered for simplicity )

        let px2 = -1.; // previous position of second pendulum sphere - x offset
        let py2 = -1.; // previos position of second pendulum sphere - y offset
        let cx = ANCHO as f32 / 2.;
        let cy = 200.; //centre of x and y for background

        let canvas = create_graphics(ANCHO, ALTO); // canvas is just a variable name DO NOT CONFUSE IT WITH P5.JS

        engine.param.ancho = ANCHO as f32;
        engine.param.alto = ALTO as f32;
        Sketch {
            engine,
            r1,
            r2,
            m1,
            m2,
            a1,
            a2,
            a1_v,
            a2_v,
            g,

            px2,
            py2,
            cx,
            cy,
            canvas,
        }
    }
    pub fn pre_load(&mut self) {
//full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        self.canvas.begin_draw();
        self.canvas.background();
        self.canvas.end_draw();
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, _canvas: &mut Canvas<Window>) {}
}
