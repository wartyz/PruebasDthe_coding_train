use libfinal::color::{background, fill3, no_stroke};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::frame_rate;
use libfinal::matem::{PVector3, pvector3, random};
use libfinal::parametros::CodigosRaton::{Derecho, Izquierdo};
use libfinal::shape::ellipse;

use crate::vehicle::Vehicle;

//use libfinal::p5::sound::SoundFile;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 640;
pub const ALTO: u32 = 360;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    vehicles: Vec<Vehicle>,
    food: Vec<PVector3>,
    poison: Vec<PVector3>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,
            vehicles: vec![],
            food: vec![],
            poison: vec![],
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self) {
        //createcanvas(&mut self.engine, ANCHO, ALTO);

        for _ in 0..50 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);
            self.vehicles.push(Vehicle::new(x, y));
        }

        for _ in 0..40 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);
            self.food.push(pvector3(x, y, 1.0));
        }

        for _ in 0..20 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);
            self.poison.push(pvector3(x, y, 1.0));
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

        if random(1.) < 0.1 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);
            self.food.push(pvector3(x, y, 1.));
        }

        if random(1.) < 0.01 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);
            self.poison.push(pvector3(x, y, 1.));
        }

        // let target = pvector3(
        //     self.engine.param.mouse_posicion.x,
        //     self.engine.param.mouse_posicion.y,
        //     1.0,
        // );

        //        fill1(127.0, &mut self.engine.param);
        //        stroke1(200.0, &mut self.engine.param);
        //        stroke_weight(2.0, &mut self.engine.param);
        //        ellipse(&mut self.engine.canvas.as_mut().unwrap(),
        //                &mut self.engine.param,
        //                target.x, target.y, 48.0, 48.0);

        for i in 0..self.food.len() {
            fill3(0.0, 255.0, 0.0, &mut self.engine.param);
            no_stroke(&mut self.engine.param);
            ellipse(
                canvas,
                &mut self.engine.param,
                self.food[i].x,
                self.food[i].y,
                4.,
                4.,
            );
        }

        for i in 0..self.poison.len() {
            fill3(255.0, 0.0, 0.0, &mut self.engine.param);
            no_stroke(&mut self.engine.param);
            ellipse(
                canvas,
                &mut self.engine.param,
                self.poison[i].x,
                self.poison[i].y,
                4.,
                4.,
            );
        }

        for i in (0..self.vehicles.len()).rev() {
            self.vehicles[i].boundaries();
            self.vehicles[i].behaviors(&mut self.food, &mut self.poison);
            self.vehicles[i].update();
            self.vehicles[i].display(&mut self.engine, canvas);

            let new_vehicle = self.vehicles[i].clonez();
            if new_vehicle.is_some() {
                self.vehicles.push(new_vehicle.unwrap());
            }

            if self.vehicles[i].dead() {
                self.vehicles.remove(i);
            }
        }

        //frame_rate(5);
    }

    pub fn mouse_dragged(&mut self) {
        if self.engine.param.mouse_boton_mantiene == Izquierdo {
            let x = self.engine.param.mouse_posicion.x;
            let y = self.engine.param.mouse_posicion.y;
            println!("mouse dragged x = {}, y _= {}", x, y);
            self.vehicles.push(Vehicle::new(x, y));
        }
    }
}
