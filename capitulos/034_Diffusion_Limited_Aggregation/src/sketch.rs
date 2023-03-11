use libfinal::color::{background, color_mode};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::parametros::ColorMode;
use crate::walker::Walker;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    tree: Vec<Walker>,
    walkers: Vec<Walker>,

    //r: f32,
    max_walkers: usize,
    iterations: usize,
    radius: f32,

    hu: f32,

    shrink: f32,
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
            tree: vec![],
            walkers: vec![],

            //r: 2.0, // Lo pongo yo era 4.0
            max_walkers: 50,
            iterations: 1000,

            radius: 8.0,

            hu: 0.0,
            shrink: 0.995,
        }
    }
    pub fn pre_load(&mut self) {
        full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        color_mode(ColorMode::HSB, &mut self.engine.param);
        let w = self.engine.param.ancho;
        let h = self.engine.param.alto;

        self.tree.push(Walker::new3(
            &mut self.engine,
            w / 2.0,
            h / 2.0,
            self.radius,
        ));
        self.radius *= self.shrink;
        for _ in 0..self.max_walkers {
            self.walkers.push(Walker::new(&mut self.engine));
            self.radius *= self.shrink;
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

        for i in 0..self.tree.len() {
            self.tree[i].show(&mut self.engine, canvas);
        }

        for i in 0..self.walkers.len() {
            self.walkers[i].show(&mut self.engine, canvas);
        }

        for _ in 0..self.iterations {
            let mut i = self.walkers.len() - 1;
            //for i in 0..self.walkers.len() {
            while i > 0 && i < self.walkers.len() {
                //println!("En draw i = {:#?}", i);
                self.walkers[i].walk(&mut self.engine);
                //self.walkers[i].show(self.r, &mut self.engine);
                if self.walkers[i].check_stuck(&self.tree) {
                    self.walkers[i].set_hue(self.hu % 360.0);
                    self.hu += 2.0;
                    self.tree.push(self.walkers[i]);
                    self.walkers.remove(i);
                    i -= 1;
                }
                i -= 1;
                //println!("En draw self.walkers.len() = {:#?}", self.walkers.len());
            }
        }
        //let r = self.walkers[self.walkers.len() - 1].r;
        while self.walkers.len() < self.max_walkers && self.radius > 1.0 {
            self.radius *= self.shrink;
            self.walkers
                .push(Walker::new1(&mut self.engine, self.radius));
        }
    }
}
