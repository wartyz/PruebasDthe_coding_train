use std::process::Command;
use libfinal::color::{blue, green, pcolor3, red, vec3_color};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::image::{PImage, pimage_vacio};
use libfinal::image::TipoFiltro::GRAY;
use libfinal::matem::round;
use libfinal::parametros::Filtros::Gray;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 1024;
pub const ALTO: u32 = 512;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del sketch
    kitten: Option<PImage>,
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
            kitten: None,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self, canvas: &mut Canvas<Window>) {
        self.kitten = PImage::load_image(canvas, "resources/imagenes/kitten32.bmp");
        self.kitten.as_mut().unwrap().filter(Gray);
        let ancho = self.kitten.as_ref().unwrap().image_width;
        let alto = self.kitten.as_ref().unwrap().image_width;

        self.kitten.as_ref().unwrap().image5(canvas, &mut self.engine.param, 0, 0, ancho, alto);
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let mut kitten2 = self.kitten.clone();

        for y in 0..kitten2.as_ref().unwrap().image_height - 1 {
            for x in 1..kitten2.as_ref().unwrap().image_width - 1 {
                let pix = &kitten2.as_ref().unwrap().image[x as usize][y as usize];
                let oldr = red(pix) as f32;
                let oldg = green(pix) as f32;
                let oldb = blue(pix) as f32;
                let factor = 1.;
                let newr = round(factor * oldr / 255.) * (255. / factor);
                let newg = round(factor * oldg / 255.) * (255. / factor);
                let newb = round(factor * oldb / 255.) * (255. / factor);

                kitten2.as_mut().unwrap().image[x as usize][y as usize] = (newr as u8, newg as u8, newb as u8, 255);

                let errr = oldr - newr;
                let errg = oldg - newg;
                let errb = oldb - newb;

                let mut c = &kitten2.as_ref().unwrap().image[(x + 1) as usize][y as usize];
                let mut r = red(&c) as f32;
                let mut g = green(&c) as f32;
                let mut b = blue(&c) as f32;
                r = r + errr * 7. / 16.;
                g = g + errg * 7. / 16.;
                b = b + errb * 7. / 16.;

                kitten2.as_mut().unwrap().image[(x + 1) as usize][y as usize] = (r as u8, g as u8, b as u8, 255);

                c = &kitten2.as_ref().unwrap().image[(x - 1) as usize][(y + 1) as usize];
                r = red(&c) as f32;
                g = green(&c) as f32;
                b = blue(&c) as f32;
                r = r + errr * 3. / 16.;
                g = g + errg * 3. / 16.;
                b = b + errb * 3. / 16.;

                kitten2.as_mut().unwrap().image[(x - 1) as usize][(y + 1) as usize] = (r as u8, g as u8, b as u8, 255);

                c = &kitten2.as_ref().unwrap().image[x as usize][(y + 1) as usize];
                r = red(&c) as f32;
                g = green(&c) as f32;
                b = blue(&c) as f32;
                r = r + errr * 5. / 16.;
                g = g + errg * 5. / 16.;
                b = b + errb * 5. / 16.;

                kitten2.as_mut().unwrap().image[x as usize][(y + 1) as usize] = (r as u8, g as u8, b as u8, 255);

                c = &kitten2.as_ref().unwrap().image[(x + 1) as usize][(y + 1) as usize];
                r = red(&c) as f32;
                g = green(&c) as f32;
                b = blue(&c) as f32;
                r = r + errr * 1. / 16.;
                g = g + errg * 1. / 16.;
                b = b + errb * 1. / 16.;

                kitten2.as_mut().unwrap().image[(x + 1) as usize][(y + 1) as usize] = (r as u8, g as u8, b as u8, 255);
            }
        }

        let ancho = kitten2.as_ref().unwrap().image_width;
        let alto = kitten2.as_ref().unwrap().image_height;

        kitten2.as_ref().unwrap().image5(canvas, &mut self.engine.param, 512, 0, ancho, alto);
    }
}
