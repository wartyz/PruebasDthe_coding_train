use std::f32::consts::PI;
use libfinal::color::{background, fill1, stroke1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::rendering::{create_graphics, PGraphics};
use libfinal::shape::{ellipse, line, stroke_weight};
use libfinal::transform::translate;

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

    pgraphics: PGraphics,
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

        let pgraphics = create_graphics(); // canvas is just a variable name DO NOT CONFUSE IT WITH P5.JS

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
            pgraphics,
        }
    }
    pub fn pre_load(&mut self) {
//full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        // self.canvas.begin_draw();
        // self.canvas.background();
        // self.canvas.end_draw();
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
        translate(self.cx, self.cy, &mut self.engine.param);
        self.pgraphics.presenta_pixels(canvas, &mut self.engine.param);

        // numerators are moduled
        let mut num1: f32 = -self.g * (2. * self.m1 + self.m2) * (self.a1).sin();
        let mut num2: f32 = -self.m2 * self.g * (self.a1 - 2. * self.a2).sin();
        let mut num3: f32 = -2. * (self.a1 - self.a2).sin() * self.m2;
        let mut num4: f32 = self.a2_v * self.a2_v * self.r2 + self.a1_v * self.a1_v * self.r1 * (self.a1 - self.a2).cos();
        let mut den: f32 = self.r1 * (2. * self.m1 + self.m2 - self.m2 * (2. * self.a1 - 2. * self.a2).cos());
        let a1_a: f32 = (num1 + num2 + num3 * num4) / den;

        num1 = 2. * (self.a1 - self.a2).sin();
        num2 = self.a1_v * self.a1_v * self.r1 * (self.m1 + self.m2);
        num3 = self.g * (self.m1 + self.m2) * (self.a1).cos();
        num4 = self.a2_v * self.a2_v * self.r2 * self.m2 * (self.a1 - self.a2).cos();
        den = self.r2 * (2. * self.m1 + self.m2 - self.m2 * (2. * self.a1 - 2. * self.a2).cos());
        let a2_a: f32 = (num1 * (num2 + num3 + num4)) / den;

        stroke1(0., &mut self.engine.param);
        stroke_weight(2, &mut self.engine.param);

        let x1: f32 = self.r1 * (self.a1).sin();
        let y1 = self.r1 * (self.a1).cos();

        let x2 = x1 + self.r2 * (self.a2).sin();
        let y2 = y1 + self.r2 * (self.a2).cos();

        line(canvas, &mut self.engine.param, 0., 0., x1, y1);
        fill1(0., &mut self.engine.param);
        ellipse(canvas, &mut self.engine.param, x1, y1, self.m1, self.m1);

        line(canvas, &mut self.engine.param, x1, y1, x2, y2);
        fill1(0., &mut self.engine.param);
        ellipse(canvas, &mut self.engine.param, x2, y2, self.m2, self.m2);

        self.a1_v += a1_a;
        self.a2_v += a2_a;
        self.a1 += self.a1_v;
        self.a2 += self.a2_v;
        // as momentum increases  , slowly pendulum comes to rest
        // a1_v *= 0.99; // for drag
        // a2_v *= 0.99; // for drag

        self.pgraphics.set_points(self.px2, self.py2, x2, y2);

        self.px2 = x2;
        self.py2 = y2;
    }
}
