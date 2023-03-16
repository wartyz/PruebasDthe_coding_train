use libfinal::color::{background, stroke1, stroke3};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{PVector3, pvector3, random};
use libfinal::parametros::CodigosRaton::Izquierdo;
use libfinal::parametros::Parametros;
use libfinal::shape::{point, stroke_weight};
use crate::particle::Particle;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 800;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del sketch
    attractors: Vec<PVector3>,
    particles: Vec<Particle>,
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

        let particles = vec![];

        let attractors = vec![];

        Sketch {
            engine,
            attractors,
            particles,
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
        background(canvas, &mut self.engine, 51);
        stroke1(255.0, &mut self.engine.param);
        stroke_weight(4, &mut self.engine.param);

        self.particles
            .push(Particle::new(random(ANCHO as f32), random(ALTO as f32)));
        //particles.push(Particle::new(200.0, 200.0));

        //point(d, &mut self.engine.param, self.attractor.x, self.attractor.y);

        // Dibuja attractors
        for i in 0..self.attractors.len() {
            stroke3(0.0, 255.0, 0.0, &mut self.engine.param);
            point(
                canvas,
                &mut self.engine.param,
                self.attractors[i].x,
                self.attractors[i].y,
            );
        }

        for i in 0..self.particles.len() {
            for j in 0..self.attractors.len() {
                self.particles[i].attracted(self.attractors[j]);
            }

            self.particles[i].update();
            self.particles[i].show(canvas, &mut self.engine.param);
        }
    }

    pub fn mouse_pressed(&mut self) {
        if self.engine.param.mouse_boton_mantiene == Izquierdo {
            self.attractors.push(pvector3(
                self.engine.param.mouse_posicion.x,
                self.engine.param.mouse_posicion.y,
                0.0,
            ));
        }
    }
}
