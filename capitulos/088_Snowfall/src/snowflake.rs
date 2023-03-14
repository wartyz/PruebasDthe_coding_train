use std::f32::consts::PI;
use libfinal::color::stroke1;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::image::{PImage, renderiza};
use libfinal::matem::{constrain, pow, PVector3, pvector3, random, random_i32_range, random_range};
use libfinal::parametros::ImageMode;
use libfinal::shape::stroke_weight;
use libfinal::transform::{pop_matrix, push_matrix, rotate, rotate_z, translate};
use crate::sketch::{ALTO, ANCHO};

pub struct Snowflake {
    img: PImage,
    pos: PVector3,
    vel: PVector3,
    acc: PVector3,
    angle: f32,
    dir: f32,
    xoff: f32,
    r: f32,

}

impl Snowflake {
    pub fn new3(sx: f32, sy: f32, simg: PImage) -> Snowflake {
        let x = sx;
        let y = sy;
        let img = simg;
        let pos = pvector3(x, y, 0.);
        let vel = pvector3(0., 0., 0.);
        let acc = pvector3(0., 0., 0.);
        let angle = random(PI as f32 * 2.0);
        let mut dir = 1.;
        if random(1.0) > 0.5 {
            dir = -1.;
        }
        let xoff = 0.;
        let r = get_random_size();

        Snowflake {
            img,
            pos,
            vel,
            acc,
            angle,
            dir,
            xoff,
            r,

        }
    }

    pub fn apply_force(&mut self, force: PVector3) {
        // Parallax effect hack
        let mut f = force;
        f.mult(self.r);

        self.acc.add(&force);
    }

    pub fn randomize(&mut self) {
        let x = random(ANCHO as f32);
        let y = random_range(-100.0, -10.0);

        self.pos = pvector3(x, y, 0.0);
        self.vel = pvector3(0.0, 0.0, 0.0);
        self.acc = pvector3(0.0, 0.0, 0.0);
        self.r = get_random_size();
    }

    pub fn update(&mut self) {
        self.xoff = (self.angle).sin() * self.r * 2.0;
        //
        // self.vel.add(&self.acc);

        self.vel.add(&self.acc);
        self.vel.limit(self.r * 0.2);

        if self.vel.mag() < 1.0 {
            self.vel.normalize();
        }

        self.pos.add(&self.vel);
        self.acc.mult(0.0);

        if self.pos.y > ALTO as f32 + self.r {
            self.randomize();
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>, engine: &mut Engine) {
        push_matrix(&mut engine.param);
        translate(self.pos.x + self.xoff, self.pos.y, &mut engine.param);

        rotate_z(self.angle, &mut engine.param);
        PImage::image_mode(&mut engine.param, ImageMode::Center);

        self.img.image5(canvas, &mut engine.param, 0, 0, self.r as u32, self.r as u32);

        pop_matrix(&mut engine.param);
    }
}

// En video usa randomgaussian()
fn get_random_size() -> f32 {
    let r = pow(random_range(0.0, 1.0), 3);
    constrain(r * 32.0, 2.0, 32.0)
}
