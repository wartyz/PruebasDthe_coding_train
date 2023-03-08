//use libfinal::matem::*;
use libfinal::color::{fill, no_stroke, stroke1, PColor, pcolor4};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::{mapa, random, random_range};
use libfinal::parametros::Parametros;
use libfinal::shape::*;

#[derive(Debug, Copy, Clone)]
pub struct Star {
    x: f32,
    y: f32,
    z: f32,
    pz: f32,
}

impl Star {
    pub fn new(param: &Parametros) -> Star {
        let z = random(param.ancho / 2.0);

        Star {
            x: random_range(-param.ancho, param.ancho),
            y: random_range(-param.alto, param.alto),

            z,
            pz: z,
        }
    }

    pub fn update(&mut self, engine: &Engine, speed: f32) {
        self.z -= speed;

        if self.z < 1.0 {
            self.z = engine.param.ancho / 2.0;
            self.x = random_range(-engine.param.ancho / 2.0, engine.param.ancho / 2.0);
            self.y = random_range(-engine.param.alto / 2.0, engine.param.alto / 2.0);

            self.pz = self.z;
        }
    }

    pub fn show(mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        fill(pcolor4(255, 255, 0, 255), &mut engine.param);
        no_stroke(&mut engine.param);

        let sx = mapa(self.x / self.z, 0.0, 1.0, 0.0, engine.param.ancho / 2.0);
        let sy = mapa(self.y / self.z, 0.0, 1.0, 0.0, engine.param.alto / 2.0);

        let r = mapa(self.z, 0.0, engine.param.ancho / 2.0, 16.0, 0.0);

        ellipse(canvas, &mut engine.param, sx, sy, r, r);

        let px = mapa(self.x / self.pz, 0.0, 1.0, 0.0, engine.param.ancho / 2.0);
        let py = mapa(self.y / self.pz, 0.0, 1.0, 0.0, engine.param.alto / 2.0);

        self.pz = self.z;

        stroke1(255.0, &mut engine.param);
        line(canvas, &mut engine.param, px, py, sx, sy);
    }
}
