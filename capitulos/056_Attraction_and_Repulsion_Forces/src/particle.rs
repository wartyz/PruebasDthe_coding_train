use libfinal::color::stroke2;
use libfinal::engine::{Canvas, Window};
use libfinal::matem::{constrain, PVector3, pvector3, random2d, sub_s};
use libfinal::parametros::Parametros;
use libfinal::shape::{line, stroke_weight};

pub struct Particle {
    pos: PVector3,
    prev: PVector3,
    vel: PVector3,
    acc: PVector3,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle {
        //et mut vel = Vector2::random2d_s();
        //vel.set_mag(random_range(2.0, 5.0));
        Particle {
            pos: pvector3(x, y, 0.0),
            prev: pvector3(x, y, 0.0),
            vel: random2d(),
            acc: pvector3(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.vel.add(&self.acc);
        self.vel.limit(5.0);
        self.pos.add(&self.vel);
        self.acc.mult(0.0);
    }

    pub fn show(&mut self, canvas: &mut Canvas<Window>, param: &mut Parametros) {
        stroke2(255.0, 255.0, param);
        stroke_weight(4, param);
        //point(d, param, self.pos.x, self.pos.y);
        line(canvas, param, self.pos.x, self.pos.y, self.prev.x, self.prev.y);

        self.prev.x = self.pos.x;
        self.prev.y = self.pos.y;
    }

    pub fn attracted(&mut self, target: PVector3) {
        // dir = target - this.pos
        let mut force = sub_s(&target, &self.pos);
        let mut d = force.mag();

        d = constrain(d, 0.1, 25.0);
        let g = 50.0; // 6.67408;
        let strength = g / (d * d);
        force.set_mag(strength);
        if d < 20.0 {
            force.mult(-10.0);
        }

        self.acc.add(&force);
    }
}
