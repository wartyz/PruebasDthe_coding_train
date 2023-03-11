use libfinal::color::{background, fill1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::parametros::CodigosTecla;
use libfinal::typography::{text, text_size};
use crate::paddle::Paddle;
use crate::puck::Puck;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    puck: Puck,

    //ding: SoundFile,
    left: Paddle,
    right: Paddle,

    left_score: i32,
    right_score: i32,
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
            puck: Puck::new(),

            //ding: SoundFile::new(String::from("resources/sonidos/22.wav")),
            left: Paddle::new(true),
            right: Paddle::new(false),
            left_score: 0,
            right_score: 0,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {}

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        background(canvas, &mut self.engine, 0);

        //self.puck.check_paddle(left);
        self.puck.check_paddle_right(self.right);
        self.puck.check_paddle_left(self.left);

        self.left.show(&mut self.engine, canvas);
        self.right.show(&mut self.engine, canvas);
        self.left.update();
        self.right.update();

        self.puck.update();
        self.puck.edges(&mut self.left_score, &mut self.right_score);
        self.puck.show(&mut self.engine, canvas);

        fill1(255.0, &mut self.engine.param);
        text_size(&mut self.engine.param, 32);
        text(
            &mut self.engine,
            canvas,
            &self.left_score.to_string(),
            32.0,
            40.0,
        );
        text(
            &mut self.engine,
            canvas,
            &self.right_score.to_string(),
            (ANCHO - 64) as f32,
            40.0,
        );
    }

    pub fn key_pressed(&mut self) {
        if self.engine.param.key == CodigosTecla::A {
            self.left.mover(-10.0);
        } else if self.engine.param.key == CodigosTecla::Z {
            self.left.mover(10.0)
        } else if self.engine.param.key == CodigosTecla::J {
            self.right.mover(-10.0)
        } else if self.engine.param.key == CodigosTecla::M {
            self.right.mover(10.0)
        }
    }

    pub fn key_released(&mut self) {
        if self.engine.param.keyr == CodigosTecla::A {
            self.left.mover(0.0);
        } else if self.engine.param.keyr == CodigosTecla::Z {
            self.left.mover(0.0)
        } else if self.engine.param.keyr == CodigosTecla::J {
            self.right.mover(0.0)
        } else if self.engine.param.keyr == CodigosTecla::M {
            self.right.mover(0.0)
        }
    }
}
