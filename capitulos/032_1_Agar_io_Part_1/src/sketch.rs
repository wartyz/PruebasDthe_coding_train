use libfinal::color::background;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{lerp, random_range};
use libfinal::transform::{scale1, translate};

use crate::blob::Blob;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    blob: Option<Blob>,
    blobs: Vec<Blob>,
    zoom: f32,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        engine.param.ancho = ANCHO as f32;
        engine.param.alto = ALTO as f32;
        Sketch {
            engine,
            blob: None,
            blobs: vec![],
            zoom: 1.0,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        let w = self.engine.param.ancho;
        let h = self.engine.param.alto;
        self.blob = Some(Blob::new(&mut self.engine, 0.0, 0.0, 64.0));
        for _ in 0..200 {
            let x = random_range(-w, w);
            let y = random_range(-h, h);
            self.blobs.push(Blob::new(&mut self.engine, x, y, 16.0));
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        background(canvas, &mut self.engine, 0);

        // Tengo que crear esta variable
        let b = self.blob.as_mut().unwrap();
        let h = self.engine.param.alto;
        let w = self.engine.param.ancho;

        //        translate(self.engine.param.ancho_pantalla / 2.0 - b.pos.x,
        //                  self.engine.param.alto_pantalla / 2.0 - b.pos.y, &mut self.engine.param);

        //println!("en draw   w = {:#?}", w);

        translate(w / 2.0, h / 2.0, &mut self.engine.param);
        let newzoom = 64.0 / b.r;

        self.zoom = lerp(self.zoom, newzoom, 0.1);

        scale1(self.zoom, &mut self.engine.param);
        translate(-b.pos.x, -b.pos.y, &mut self.engine.param);

        //println!("en draw matriz_total = {:#?}", self.engine.param.matriz_total);
        //println!("En draw self.blobs.len() = {:#?}", self.blobs.len());
        for i in (0..(self.blobs.len() - 1)).rev() {
            self.blobs[i].show(&mut self.engine, canvas);
            if b.eats(&mut self.blobs[i]) {
                // blobbs[i].splice(0,1)
                self.blobs.remove(i);
            }
            //self.blobs[i].show(&mut self.engine);
        }

        b.show(&mut self.engine, canvas);
        b.update(&mut self.engine.param);
        //
        //        for b in &mut self.blobs {
        //            b.show(&mut self.engine);
        //        }
        //        let a = self.n * 137.5;
        //
        //        let r = self.c * self.n.sqrt();
        //
        //        let x = r * a.cos() + self.engine.param.ancho_pantalla / 2.0;
        //        let y = r * a.sin() + self.engine.param.alto_pantalla / 2.0;
        //
        //        //fill1(255, &mut self.engine.param);
        //        fill3(((a - r) % 255.0) as u8, 255, 255, &mut self.engine.param);
        //        no_stroke(&mut self.engine.param);
        //        ellipse(&mut self.engine.canvas.as_mut().unwrap(), &mut self.engine.param, x, y, 4.0, 4.0);
        //
        //
        //        self.n += 1.0;
    }
}
