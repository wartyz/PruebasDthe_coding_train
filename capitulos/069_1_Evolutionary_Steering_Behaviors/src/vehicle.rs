use std::f32::consts::PI;
use libfinal::color::{fill, lerp_color, pcolor3, pcolor4, stroke, stroke3};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::{PVector3, pvector3, random, random_range, sub_s};

use libfinal::parametros::{ModosBeginShape, Parametros};
use libfinal::shape::{begin_shape, end_shape, line, stroke_weight, vertex};

use libfinal::structure::{pop, push};
use libfinal::transform::{pop_matrix, push_matrix, rotate_z, translate};
use crate::sketch::{ALTO, ANCHO};

pub struct Vehicle {
    acceleration: PVector3,
    velocity: PVector3,
    position: PVector3,
    r: f32,

    maxspeed: f32,
    maxforce: f32,

    health: f32,

    dna: Option<[f32; 4]>,
}

impl Vehicle {
    pub fn new(x: f32, y: f32) -> Vehicle {
        let dna = None;
        Vehicle::new3(x, y, dna)
    }

    pub fn new3(x: f32, y: f32, dnax: Option<[f32; 4]>) -> Vehicle {
        let mr = 0.01;
        let mut dna = [0.; 4];
        if dnax.is_none() {
            // Food weight
            dna[0] = random_range(-2., 2.);
            // Poison weight
            dna[1] = random_range(-2., 2.);
            // Food perception
            dna[2] = random_range(0., 100.);
            // Poision Percepton
            dna[3] = random_range(0., 100.);
        } else {
            // Mutation
            //dbg!(random_range(-0.1, 0.1));
            dna[0] = dnax.unwrap()[0];
            if random(1.) < mr {
                dna[0] += random_range(-0.1, 0.1);
            }
            dna[1] = dnax.unwrap()[1];
            if random(1.) < mr {
                dna[1] += random_range(-0.1, 0.1);
            }
            dna[2] = dnax.unwrap()[2];
            if random(1.) < mr {
                dna[2] += random_range(-10., 10.);
            }
            dna[3] = dnax.unwrap()[3];
            if random(1.) < mr {
                dna[3] += random_range(-10., 10.);
            }
        }
        Vehicle {
            acceleration: pvector3(0., 0., 1.),
            velocity: pvector3(0., -2., 1.),
            position: pvector3(x, y, 1.),
            r: 4.0,

            maxspeed: 5.,
            maxforce: 0.5,

            health: 1.0,

            dna: Some(dna),
        }
    }

    pub fn update(&mut self) {
        self.health -= 0.005;
        // Update vlocity
        self.velocity.add(&self.acceleration);
        // Limit speed
        self.velocity.limit(self.maxspeed);
        self.position.add(&self.velocity);
        // Reset accelerationelertion to 0 each cycle
        self.acceleration.mult(0.);
    }

    pub fn applyforce(&mut self, force: PVector3) {
        // We could add mass here if we want A = F
        self.acceleration.add(&force)
    }

    pub fn behaviors(&mut self, good: &mut Vec<PVector3>, bad: &mut Vec<PVector3>) {
        let mut steer_g = self.eat(good, 0.2, self.dna.unwrap()[2]);
        let mut steer_b = self.eat(bad, -1., self.dna.unwrap()[3]);

        steer_g.mult(self.dna.unwrap()[0]);
        steer_b.mult(self.dna.unwrap()[1]);

        self.applyforce(steer_g);
        self.applyforce(steer_b);
    }

    pub fn clonez(&mut self) -> Option<Vehicle> {
        if random(1.) < 0.002 {
            return Some(Vehicle::new3(self.position.x, self.position.y, self.dna));
        } else {
            return None;
        }
    }
    pub fn eat(&mut self, list: &mut Vec<PVector3>, nutrition: f32, perception: f32) -> PVector3 {
        let mut record = f32::MAX;
        let mut closest: PVector3 = pvector3(0.0, 0.0, 1.0);
        for i in (0..list.len()).rev() {
            let d = self.position.dist(&list[i]);

            if d < self.maxspeed {
                list.remove(i);
                self.health += nutrition;
            } else {
                if d < record && d < perception {
                    record = d;
                    closest = list[i];
                }
            }
        }

        // This is the moment of eating
        if closest.x != 0. || closest.y != 0. {
            return closest;
        }

        pvector3(0.0, 0.0, 1.0)
    }

    pub fn seek(&mut self, target: PVector3) -> PVector3 {
        // A vector pointing from the location to the target
        let mut desired = sub_s(&target, &self.position);

        // Scale to maximum speed
        desired.set_mag(self.maxspeed);

        // Steering = Desired minus velocity
        let mut steer = sub_s(&desired, &self.velocity);
        steer.limit(self.maxforce); // Limit to max

        steer
        //self.applyforce(steer);
    }

    pub fn dead(&mut self) -> bool {
        self.health < 0.
    }

    pub fn display(&mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        // Draw a triangle rotated in the direction of velocity
        let angle = self.velocity.heading() + PI / 2.;

        push_matrix(&mut engine.param);
        translate(self.position.x, self.position.y, &mut engine.param);
        rotate_z(angle, &mut engine.param);

        let gr = pcolor3(0, 255, 0);
        let rd = pcolor3(255, 0, 0);
        let col = lerp_color(rd, gr, self.health);

        // stroke3(0.0, 255.0, 0.0, &mut engine.param);
        // line(
        //     canvas,
        //     &mut engine.param,
        //     0.0,
        //     0.0,
        //     0.0,
        //     -self.dna.unwrap()[0] * 20.0,
        // );
        // stroke3(255.0, 0.0, 0.0, &mut engine.param);
        // line(
        //     canvas,
        //     &mut engine.param,
        //     0.0,
        //     0.0,
        //     0.0,
        //     -self.dna.unwrap()[1] * 20.0,
        // );

        fill(col, &mut engine.param);
        stroke(col, &mut engine.param);
        stroke_weight(1, &mut engine.param);
        begin_shape(ModosBeginShape::NadaShape);
        vertex(0., -self.r * 2., &mut engine.param);
        vertex(-self.r, self.r * 2., &mut engine.param);
        vertex(self.r, self.r * 2., &mut engine.param);

        end_shape(canvas, &mut engine.param, ModosBeginShape::Close);

        pop_matrix(&mut engine.param);
    }

    pub fn boundaries(&mut self) {
        let d: f32 = 25.;

        let mut desired: Option<PVector3> = None;

        if self.position.x < d {
            desired = Some(pvector3(self.maxspeed, self.velocity.y, 1.));
        } else if self.position.x > ANCHO as f32 - d {
            desired = Some(pvector3(-self.maxspeed, self.velocity.y, 1.));
        }

        if self.position.y < d {
            desired = Some(pvector3(self.velocity.x, self.maxspeed, 1.));
        } else if self.position.y > ALTO as f32 - d {
            desired = Some(pvector3(self.velocity.x, -self.maxspeed, 1.));
        }

        if desired.is_some() {
            let mut d = desired.unwrap();
            d.normalize();
            d.mult(self.maxspeed);
            let mut steer = sub_s(&d, &self.velocity);
            steer.limit(self.maxforce);
            self.applyforce(steer);
        }
    }
}
