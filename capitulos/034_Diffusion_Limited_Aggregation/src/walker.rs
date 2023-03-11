use libfinal::color::{fill3, fill4, no_stroke};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::{constrain, floor, PVector3, pvector3, random, random2d};
use libfinal::shape::ellipse;

#[derive(Debug, Clone, Copy)]
pub struct Walker {
    pos: PVector3,
    stuck: bool,
    pub r: f32,
    hu: f32,
}

impl Walker {
    pub fn new(engine: &mut Engine) -> Walker {
        Walker {
            pos: random_point(engine),
            stuck: false,
            r: 8.0,
            hu: 0.0,
        }
    }
    pub fn new1(engine: &mut Engine, radius: f32) -> Walker {
        Walker {
            pos: random_point(engine),
            stuck: false,
            r: radius,
            hu: 0.0,
        }
    }
    pub fn new3(engine: &mut Engine, x: f32, y: f32, radius: f32) -> Walker {
        Walker {
            pos: pvector3(x, y, 1.0),
            stuck: true,
            r: radius,
            hu: 0.0,
        }
    }
    pub fn new4(engine: &mut Engine, x: f32, y: f32, stuck: bool, radius: f32) -> Walker {
        Walker {
            pos: pvector3(x, y, 1.0),
            stuck,
            r: x,
            hu: 0.0,
        }
    }

    pub fn walk(&mut self, engine: &mut Engine) {
        let vel = random2d();
        //let vel = Vector2::new(random_range(-1.0, 1.0), random_range(-0.5, 1.0));
        self.pos.add(&vel);
        self.pos.x = constrain(self.pos.x, 0.0, engine.param.ancho);
        self.pos.y = constrain(self.pos.y, 0.0, engine.param.alto);
    }

    pub fn check_stuck(&mut self, others: &Vec<Walker>) -> bool {
        for i in 0..others.len() {
            let d = dist_sq(&self.pos, &others[i].pos);
            if d < (self.r * self.r + others[i].r * others[i].r + 2.0 * others[i].r * self.r) {
                //if random(1.0) < 0.1 {
                self.stuck = true;
                return true;
                //break;
                //}
            }
        }
        return false;
    }

    pub fn set_hue(&mut self, hu: f32) {
        self.hu = hu;
    }

    pub fn show(&mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        let p = &mut engine.param; // Nuevo invento ***********
        no_stroke(p);
        if self.stuck {
            fill4(self.hu, 255.0, 100.0, 200.0, p);
        } else {
            fill3(360.0, 0.0, 255.0, p);
        }

        ellipse(
            canvas,
            p,
            self.pos.x,
            self.pos.y,
            self.r * 2.0,
            self.r * 2.0,
        );
    }
}

pub fn random_point(engine: &mut Engine) -> PVector3 {
    let w = engine.param.ancho;
    let h = engine.param.alto;

    let i = floor(random(4.0)) as i32;

    match i {
        0 => {
            let x = random(w);
            pvector3(x, 0.0, 1.0)
        }
        1 => {
            let x = random(w);
            pvector3(x, h, 1.0)
        }
        2 => {
            let y = random(h);
            pvector3(0.0, y, 1.0)
        }
        3 => {
            let y = random(h);
            pvector3(w, y, 1.0)
        }
        _ => {
            panic!("fallo en random_point")
        }
    }
}

pub fn dist_sq(a: &PVector3, b: &PVector3) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    dx * dx + dy * dy
}
