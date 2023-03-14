use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{dist_s, PVector4, pvector4};
use crate::boxy::Boxy;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,
    angulo_camara: f32,
    distancia_camara: f32,

    // Variables globales del scketch
    a: f32,

    sponge: Vec<Boxy>,
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

        let distancia_camara = PVector4::dist_s(
            &PVector4::default(),
            &pvector4(
                engine.param.camara.posx,
                engine.param.camara.posy,
                engine.param.camara.posz,
                1.0,
            ),
        );
        Sketch {
            engine,
            angulo_camara: 0.0,
            distancia_camara,
            a: 0.0,

            sponge: vec![Boxy::new(0.0, 0.0, 0.0, 200.0)],
        }
    }
    pub fn pre_load(&mut self) {
        full_screen(&mut self.engine);
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
    pub fn draw(&mut self, _canvas: &mut Canvas<Window>) {}
}
