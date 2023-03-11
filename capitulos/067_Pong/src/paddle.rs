use libfinal::color::fill1;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::constrain;
use libfinal::parametros::RectMode;
use libfinal::shape::{rect, rect_mode};
use crate::sketch::ALTO;
use crate::sketch::ANCHO;

#[derive(Debug, Copy, Clone)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    ychange: f32,
}

impl Paddle {
    pub fn new(left: bool) -> Paddle {
        let w = 20.0;
        let mut x;
        if left {
            x = w;
        } else {
            x = ANCHO as f32 - w;
        }
        Paddle {
            x,
            y: ALTO as f32 / 2.0,
            w,
            h: 100.0,

            ychange: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.y += self.ychange;
        self.y = constrain(self.y, self.h / 2.0, ALTO as f32 - self.h / 2.0);
    }

    pub fn mover(&mut self, steps: f32) {
        self.ychange = steps;
    }

    pub fn show(&mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        fill1(255.0, &mut engine.param);
        rect_mode(RectMode::Center, &mut engine.param);
        rect(canvas, &mut engine.param, self.x, self.y, self.w, self.h);
    }
}
