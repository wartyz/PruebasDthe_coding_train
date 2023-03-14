use libfinal::color::background;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::image::{PImage, pimage_vacio};
use libfinal::matem::{floor, PVector3, pvector3, random};

use crate::snowflake::Snowflake;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 800;
pub const ALTO: u32 = 600;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del sketch
    snow: Vec<Snowflake>,
    textures: Option<Vec<PImage>>,
    gravity: PVector3,
    z_off: f32,

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
            snow: vec![],
            textures: None,
            gravity: pvector3(0.0, 0.03, 1.0),
            z_off: 0.,

        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self, imagen: &mut PImage) {
        for x in (0..imagen.image_width as i32).step_by(32) {
            for y in (0..imagen.image_height as i32).step_by(32) {

                // obtiene el trozo de imagen de el buffer
                let img: PImage = imagen.get_trozo(x as usize, y as usize);

                if self.textures.is_none() {
                    //dbg!("None");
                    let mut v = vec![];
                    v.push(img);
                    self.textures = Some(v);
                } else {
                    //dbg!("Some");
                    self.textures.as_mut().unwrap().push(img);
                }
            }
        }

        for _ in 0..400 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);
            let design_index = floor(random(self.textures.as_ref().unwrap().len() as f32)) as usize;
            let design: PImage = self.textures.as_ref().unwrap()[design_index].clone();
            self.snow.push(Snowflake::new3(x, y, design));
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

        for flake in &mut self.snow.iter_mut() {
            flake.apply_force(self.gravity);
            flake.update();
            flake.render(canvas, &mut self.engine);
        }
    }
}
