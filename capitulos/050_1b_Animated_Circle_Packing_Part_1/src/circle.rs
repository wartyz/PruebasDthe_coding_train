use libfinal::color::{no_fill, stroke1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::parametros::Parametros;
use libfinal::shape::{ellipse, stroke_weight};

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,

    pub growing: bool,
}

impl Circle {
    pub fn new(x: f32, y: f32) -> Circle {
        Circle {
            x,
            y,
            r: 1.0,
            growing: true,
        }
    }

    pub fn grow(&mut self) {
        if self.growing {
            self.r += 0.5;
        }
    }

    pub fn edges(&self, param: &mut Parametros) -> bool {
        (self.x + self.r) > param.ancho
            || (self.x - self.r) < 0.0
            || (self.y + self.r) > param.alto
            || (self.y - self.r) < 0.0
    }

    //pub fn show(&mut self, param: &mut Parametros, engine: &mut Engine) {
    pub fn show(&self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        let p = &mut engine.param; // Nuevo invento ***********

        stroke1(255.0, p);

        stroke_weight(2, p);
        no_fill(p);
        ellipse(canvas, p, self.x, self.y, self.r, self.r);
    }
}
