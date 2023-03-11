use std::cell::RefCell;
use std::rc::Rc;
use libfinal::color::background;
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use crate::segment::Segment;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    tentacle: Option<Rc<RefCell<Segment>>>,
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
            tentacle: None,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        let mut len = 50.0;
        let seg1 = Segment::new4(300.0, 200.0, len, 0.0f32.to_radians());
        self.tentacle = Some(Rc::new(RefCell::new(seg1)));

        let mut current = self.tentacle.clone();
        for _ in 0..20 {
            len *= 0.75;
            let next = Segment::new3(current.clone(), len, 0.0f32.to_radians());

            current.as_ref().unwrap().borrow_mut().child =
                Some(Rc::new(RefCell::new(next.clone())));
            current = Some(Rc::new(RefCell::new(next)));
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
        background(canvas, &mut self.engine, 51);

        let mut next = self.tentacle.clone();
        while next.is_some() {
            let kk = next.as_ref().unwrap();

            kk.borrow_mut().wiggle();
            kk.borrow_mut().update();
            kk.borrow_mut().show(&mut self.engine, canvas);
            next = next.unwrap().borrow_mut().child.clone();
        }
    }
}
