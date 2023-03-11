use libfinal::color::{background, color_mode, stroke3};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{dist_s, floor, PVector3, pvector3, random, random2d, random_range};
use libfinal::parametros::ColorMode;
use libfinal::shape::{point, stroke_weight};

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch

    r: f32,
    k: i32,
    grid: Vec<Option<PVector3>>,
    w: f32,
    active: Vec<PVector3>,
    cols: i32,
    rows: i32,
    ordered: Vec<PVector3>,
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

        let r = 4.0;
        Sketch {
            engine,
            r: r,
            k: 30,
            grid: vec![],
            w: r / 2.0_f32.sqrt(),
            active: vec![],
            cols: 0,
            rows: 0,
            ordered: vec![],
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self, canvas: &mut Canvas<Window>) {
        let w = self.engine.param.ancho;
        let h = self.engine.param.alto;

        background(canvas, &mut self.engine, 0);
        stroke_weight(4, &mut self.engine.param);
        color_mode(ColorMode::HSB, &mut self.engine.param);

        // STEP 0
        self.cols = floor(w / self.w) as i32;
        self.rows = floor(h / self.w) as i32;
        for _ in 0..(self.cols * self.rows) as usize {
            self.grid.push(None);
        }
        // STEP 1
        let x = w / 2.0;
        let y = h / 2.0;
        let i = floor(x / self.w) as i32;
        let j = floor(y / self.w) as i32;
        let pos = Some(pvector3(x, y, 1.0));
        self.grid[(i + j * self.cols) as usize] = pos.clone();
        self.active.push(pvector3(x, y, 1.0));
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
        for _ in 0..25 {
            if self.active.len() > 0 {
                let rand_index = floor(random(self.active.len() as f32)) as usize;
                let pos = self.active[rand_index].clone();
                let mut found = false;
                for _ in 0..self.k {
                    let mut sample = random2d();
                    let m = random_range(self.r, 2.0 * self.r);
                    sample.set_mag(m);
                    sample.add(&pos);

                    let col = floor(sample.x / self.w) as i32;
                    let row = floor(sample.y / self.w) as i32;

                    if col > -1 && row > -1 && col < self.cols && row < self.rows {
                        let mut ok = true;
                        for i in -1..=1 {
                            for j in -1..=1 {
                                //println!("i = {} j = {} col = {} row = {}", i, j, col, row);

                                let index = (col + i) + (row + j) * self.cols as i32;
                                //println!("index = {} self.grid.len() = {}", index, self.grid.len());
                                // Aqui da error de indice out of bounds LO PONGO YO
                                if (index >= 0) || (index < (self.grid.len() - 1) as i32) {
                                    //println!("2index = {} self.grid.len() = {}", index, self.grid.len());

                                    let neighbor = self.grid[index as usize].clone();
                                    if neighbor.is_some() {
                                        let d = dist_s(&sample, &neighbor.unwrap());
                                        if d < self.r {
                                            ok = false;
                                        }
                                    }
                                }
                            }
                        }
                        if ok {
                            found = true;
                            self.grid[(col + row * self.cols as i32) as usize] =
                                Some(sample.clone());
                            let sample2 = sample.clone();
                            self.active.push(sample);
                            self.ordered.push(sample2);
                            // Should we break?
                            //break;
                        }
                    }
                }
                if !found {
                    self.active.remove(rand_index);
                }
            }
        }
        let p = &mut self.engine.param; // Nuevo invento ***********
        for i in 0..self.ordered.len() {
            //if self.ordered[i].is_some() {  // No entiendo
            stroke3(i as f32 % 360.0, 100.0, 100.0, p);
            stroke_weight((self.r * 0.5) as u8, p);
            point(canvas, p, self.ordered[i].x, self.ordered[i].y);
            //}
        }
    }
}
