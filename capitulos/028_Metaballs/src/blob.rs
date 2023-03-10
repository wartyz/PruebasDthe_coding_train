use libfinal::color::{no_fill, stroke1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::{PVector3, random_range, random2d, pvector3};
use libfinal::shape::{ellipse, stroke_weight};
use crate::sketch::{ALTO, ANCHO};

pub struct Blob {
    pub pos: PVector3,
    pub r: f32,
    vel: PVector3,
}

impl Blob {
    pub fn new(x: f32, y: f32) -> Blob {
        let mut vel = random2d();
        vel.mult(random_range(2.0, 5.0));
        Blob {
            pos: pvector3(x, y, 1.0),
            r: 40.0,
            vel,
        }
    }

    pub fn update(&mut self) {
        self.pos.add(&self.vel);

        if self.pos.x > ANCHO as f32 || self.pos.x < 0.0 {
            self.vel.x *= -1.0;
        }

        if self.pos.y > ALTO as f32 || self.pos.y < 0.0 {
            self.vel.y *= -1.0;
        }
    }

    pub fn show(&mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        no_fill(&mut engine.param);
        stroke1(0.0, &mut engine.param);
        stroke_weight(4, &mut engine.param);
        //ellipse(param, d, self.pos.x, self.pos.y, self.r * 2.0, self.r * 2.0);

        ellipse(
            canvas,
            &mut engine.param,
            self.pos.x,
            self.pos.y,
            self.r * 2.0,
            self.r * 2.0,
        );
    }
}
