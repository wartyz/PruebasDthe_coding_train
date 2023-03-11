use libfinal::color::{color_mode, background};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{dist4, random};
use libfinal::parametros::ColorMode;
use libfinal::image::{PImage, pimage_vacio};

use crate::blob::Blob;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 640;
pub const ALTO: u32 = 360;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    blobs: Vec<Blob>,
    pimage: PImage,
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

        let mut blobs: Vec<Blob> = vec![];
        for _ in 0..10 {
            blobs.push(Blob::new(random(ANCHO as f32), random(ALTO as f32)));
        }
        Sketch { engine, blobs, pimage: pimage_vacio() }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self, canvas: &mut Canvas<Window>) {
        background(canvas, &mut self.engine, 51);
        self.pimage.load_pixels(canvas);
        color_mode(ColorMode::HSB, &mut self.engine.param)
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        for x in 0..ANCHO as usize {
            for y in 0..ALTO as usize {
                //let index = (x + y * ANCHO) as usize;
                //let index = (x + y * ANCHO) as usize;

                let mut sum: f32 = 0.0;

                for i in 0..self.blobs.len() {
                    let d = dist4(x as f32, y as f32, self.blobs[i].pos.x, self.blobs[i].pos.y);
                    sum += 100.0 * self.blobs[0].r / d;
                }
                //                self.engine.param.pixels[index] =
                //                    //Color::color_from_hsv(Vector3::new(sum % 255.0, 255.0, 255.0));
                //                    Color::new(sum as u8, sum as u8, 255, 255);
                // self.engine.param.pixels[index] = (sum % 255.0) as u8;
                // self.engine.param.pixels[index + 1] = 255;
                // self.engine.param.pixels[index + 2] = 255;
                // self.engine.param.pixels[index + 3] = 255;
                //self.pimage.image[index] = vec![((sum % 255.0) as u8, 255, 255, 255)];
                // BGRA
                self.pimage.image[x][y] = (255, 0, (sum % 255.0) as u8, 0);
            }
        }

        self.pimage.update_pixels(canvas);

        for i in 0..self.blobs.len() {
            self.blobs[i].update();
            //self.blobs[i].show(d, &mut self.engine.param);
        }
    }
}
