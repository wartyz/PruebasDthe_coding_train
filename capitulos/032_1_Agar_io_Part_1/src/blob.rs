use std::f32::consts::PI;
use libfinal::color::fill1;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::{dist_s3, PVector3, pvector3};
use libfinal::parametros::Parametros;
use libfinal::shape::ellipse;

pub struct Blob {
    pub pos: PVector3,
    pub r: f32,
    pub vel: PVector3,
}

impl Blob {
    pub fn new(engine: &mut Engine, x: f32, y: f32, r: f32) -> Blob {
        //println!("en Blob::new   w = {:#?}", w);
        Blob {
            pos: pvector3(x, y, 1.0),
            r: r,
            vel: pvector3(0.0, 0.0, 1.0),
        }
    }

    pub fn update(&mut self, param: &mut Parametros) {
        let mut newvel = pvector3(
            param.mouse_posicion.x - param.ancho / 2.0,
            param.mouse_posicion.y - param.alto / 2.0,
            1.0,
        );
        //println!("1111En Blob::update   vel = {:#?}", vel);
        //vel.sub(&self.pos);
        //println!("2222En Blob::update   vel = {:#?}", vel);
        newvel.set_mag(3.0);
        self.vel.lerp(newvel, 0.2);
        //println!("33333En Blob::update   vel = {:#?}", vel);

        self.pos.add(&self.vel);
    }

    pub fn eats(&mut self, other: &mut Blob) -> bool {
        let d = dist_s3(&self.pos, &other.pos);
        if d < (self.r + other.r) {
            let sum = PI * self.r * self.r + PI * other.r * other.r;
            self.r = (sum / PI).sqrt();
            //self.r += other.r;
            return true;
        } else {
            return false;
        }
    }

    pub fn show(&mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        fill1(255.0, &mut engine.param);
        //println!("en blob:show   matriz_total = {:#?}", engine.param.matriz_total);
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
