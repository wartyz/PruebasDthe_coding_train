use std::f32::consts::PI;
use libfinal::color::{no_fill, stroke2};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::radians;
use libfinal::shape::{ellipse, stroke_weight};

#[derive(Clone, Copy, Debug)]
pub struct Orbit {
    pub x: f32,
    pub y: f32,
    r: f32,

    n: i32,
    pub mipos: Option<usize>,
    pub parent: Option<usize>,
    pub child: Option<usize>,
    speed: f32,
    angle: f32,
}

impl Orbit {
    pub fn new3(
        x: f32,
        y: f32,
        r: f32,
        n: i32,
        resolution: f32,
        mipos: Option<usize>,
        parent: Option<usize>,
    ) -> Orbit {
        Orbit::new5(x, y, r, n, resolution, mipos, parent)
    }
    pub fn new5(
        x: f32,
        y: f32,
        r: f32,
        n: i32,
        resolution: f32,
        mipos: Option<usize>,
        parent: Option<usize>,
    ) -> Orbit {
        let k: f32 = -4.0;

        let speed = radians(k.powi(n - 1)) / resolution;
        Orbit {
            x,
            y,
            r,
            n,
            mipos,
            parent,
            child: None,
            speed,
            angle: -PI / 2.0,
        }
    }

    pub fn update(
        mut self,
        parent: Option<Orbit>,
        //canvas: &mut Canvas<Window>,
        //param: &mut Parametros,
    ) -> Orbit {
        if parent.is_some() {
            self.angle += self.speed;
            //dbg!(self.angle);
            let rsum = self.r + parent.unwrap().r;
            self.x = parent.unwrap().x + rsum * self.angle.cos();
            self.y = parent.unwrap().y + rsum * self.angle.sin();

            //ellipse(param, d, x, y, self.r * 2.0, self.r * 2.0);
        }
        self
    }
    pub fn show(&mut self, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        stroke2(255.0, 100.0, &mut engine.param);
        stroke_weight(2, &mut engine.param);
        no_fill(&mut engine.param);
        ellipse(
            canvas,
            &mut engine.param,
            self.x,
            self.y,
            self.r * 2.0,
            self.r * 2.0,
        );
    }

    pub fn add_child(&self, resolution: f32, nuevapos: Option<usize>) -> Orbit {
        let newr = self.r / 3.0;
        let newx = self.x + self.r + newr;
        let newy = self.y;
        let ch = Orbit::new5(
            newx,
            newy,
            newr,
            self.n + 1,
            resolution,
            nuevapos,
            self.mipos,
        );

        ch
    }
}
