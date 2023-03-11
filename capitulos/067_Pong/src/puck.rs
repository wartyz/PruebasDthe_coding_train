use std::f32::consts::PI;
use libfinal::color::fill1;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::{mapa, random, random_range};
use libfinal::shape::ellipse;
use crate::paddle::Paddle;
use crate::sketch::ALTO;
use crate::sketch::ANCHO;

//use libfinal::p5::sound::SoundFile;

#[derive(Debug, Copy, Clone)]
pub struct Puck {
    x: f32,
    y: f32,
    xspeed: f32,
    yspeed: f32,
    r: f32,
}

impl Puck {
    pub fn new() -> Puck {
        let mut p = Puck {
            x: ANCHO as f32 / 2.0,
            y: ALTO as f32 / 2.0,
            xspeed: 0.0,
            yspeed: 0.0,
            r: 12.0,
        };
        p.reset();
        p
    }

    pub fn check_paddle_left(&mut self, p: Paddle) {
        if self.y < (p.y + p.h / 2.0)
            && self.y > (p.y - p.h / 2.0)
            && (self.x - self.r) < (p.x + p.w / 2.0)
        {
            if self.x > p.x {
                let diff = self.y - (p.y - p.h / 2.0);
                let rad = 45.0_f32.to_radians();
                let angle = mapa(diff, 0.0, p.h, -rad, rad);
                self.xspeed = 5.0 * angle.cos();
                self.yspeed = 5.0 * angle.sin();
                self.x = p.x + p.w / 2.0 + self.r;
                //self.xspeed *= -1.0;
            }
        }
    }

    pub fn check_paddle_right(&mut self, p: Paddle) {
        if self.y < (p.y + p.h / 2.0)
            && self.y > (p.y - p.h / 2.0)
            && (self.x + self.r) > (p.x - p.w / 2.0)
        {
            if self.x < p.x {
                let diff = self.y - (p.y - p.h / 2.0);
                let rad = 135.0_f32.to_radians();
                let angle = mapa(diff, 0.0, p.h, -rad, rad);
                self.xspeed = 5.0 * angle.cos();
                self.yspeed = 5.0 * angle.sin();
                self.x = p.x - p.w / 2.0 - self.r;
                //self.xspeed *= -1.0;
            }
        }
    }

    pub fn update(&mut self) {
        self.x += self.xspeed;
        self.y += self.yspeed;
    }

    pub fn reset(&mut self) {
        self.x = ANCHO as f32 / 2.0;
        self.y = ALTO as f32 / 2.0;

        let angle = random_range(-PI / 4.0, PI / 4.0);
        self.xspeed = 5.0 * angle.cos();
        self.yspeed = 5.0 * angle.sin();

        if random(1.0) < 0.5 {
            self.xspeed *= -1.0;
        }
    }

    pub fn edges(&mut self, leftscore: &mut i32, rightscore: &mut i32) {
        if self.y < 0.0 || self.y > ALTO as f32 {
            self.yspeed *= -1.0;
        }

        if self.x - self.r > ANCHO as f32 {
            //ding.play();
            *leftscore += 1;
            self.reset();
        }

        if self.x + self.r < 0.0 {
            *rightscore += 1;
            self.reset();
        }
    }

    pub fn show(&mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        fill1(255.0, &mut engine.param);
        ellipse(
            canvas,
            &mut engine.param,
            self.x,
            self.y,
            self.r * 2.0,
            self.r * 2.0,
        );
    }
}
