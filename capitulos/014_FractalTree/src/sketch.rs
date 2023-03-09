use libfinal::color::{background, stroke1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::parametros::Parametros;
use libfinal::shape::line;
use libfinal::transform::{pop_matrix, push_matrix, rotate, rotate_z, translate};

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 400;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    angle: f32,
    branch_ratio: f32,
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
            angle: 45.,
            branch_ratio: 0.67,
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
        self.mouse_wheel();
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        background(canvas, &mut self.engine, 51);

        stroke1(255.0, &mut self.engine.param);
        translate(ANCHO as f32 / 2., ALTO as f32, &mut self.engine.param);
        self.branch(canvas, 100.);
    }

    fn branch(&mut self, canvas: &mut Canvas<Window>, lon: f32) {
        line(canvas, &mut self.engine.param, 0., 0., 0., -lon);
        translate(0., -lon, &mut self.engine.param);
        if lon > 4. {
            push_matrix(&mut self.engine.param);
            rotate_z(self.angle, &mut self.engine.param);
            self.branch(canvas, lon * self.branch_ratio);
            pop_matrix(&mut self.engine.param);
            push_matrix(&mut self.engine.param);
            rotate_z(-self.angle, &mut self.engine.param);
            self.branch(canvas, lon * self.branch_ratio);
            pop_matrix(&mut self.engine.param);
        }
    }

    fn mouse_wheel(&mut self) {
        self.angle += self.engine.param.mouse_rueda_y / 10.;
    }
}