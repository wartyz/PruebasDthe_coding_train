use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use libfinal::color::stroke1;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::{mapa, noise1, pvector2, PVector2, PVector3, pvector3, random};
use libfinal::shape::{line, stroke_weight};

#[derive(Clone)]
pub struct Segment {
    a: PVector2,
    len: f32,
    angle: f32,
    self_angle: f32,

    xoff: f32,

    pub parent: Option<Rc<RefCell<Segment>>>,
    pub child: Option<Rc<RefCell<Segment>>>,

    b: PVector2,
}

impl Segment {
    pub fn new3(parent: Option<Rc<RefCell<Segment>>>, len: f32, angle: f32) -> Segment {
        let a = parent.clone().unwrap().borrow_mut().b;
        Segment {
            a,
            len,
            angle,
            self_angle: angle,
            xoff: random(1000.0),

            parent,
            child: None,

            b: Segment::calculate_b(a, len, angle),
        }
    }

    pub fn new4(x: f32, y: f32, len: f32, angle: f32) -> Segment {
        let a = pvector2(x, y);
        Segment {
            a,
            len,
            angle,
            self_angle: angle,
            xoff: random(1000.0),

            parent: None,
            child: None,

            b: Segment::calculate_b(a, len, angle),
        }
    }

    pub fn wiggle(&mut self) {
        let maxangle = 1.0;
        let minangle = -1.0;
        // 10.0 lo pongo yo
        self.self_angle = mapa(noise1(self.xoff) / 10.0, -1.0, 1.0, maxangle, minangle);
        self.xoff += 0.03;
    }

    pub fn update(&mut self) {
        self.angle = self.self_angle;
        match &self.parent {
            Some(_) => {
                self.a = self.parent.clone().unwrap().borrow().b;

                self.angle += self.parent.clone().unwrap().borrow().angle;
            }
            None => {
                self.angle += -PI / 2.0;
            }
        }

        self.b = Segment::calculate_b(self.a, self.len, self.angle);
    }

    pub fn calculate_b(a: PVector2, len: f32, angle: f32) -> PVector2 {
        let dx = len * angle.cos();
        let dy = len * angle.sin();

        pvector2(a.x + dx, a.y + dy)
    }

    pub fn show(&mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        stroke1(255.0, &mut engine.param);
        stroke_weight(4, &mut engine.param);
        line(
            canvas,
            &mut engine.param,
            self.a.x,
            self.a.y,
            self.b.x,
            self.b.y,
        );
    }
}
