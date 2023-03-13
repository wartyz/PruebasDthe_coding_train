use libfinal::color::{background, no_fill, stroke1};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{angle_mode, cos_gr, lerp, PVector3, pvector3, sin_gr, vector3lerp};
use libfinal::parametros::ModosAngulo::Degrees;
use libfinal::parametros::ModosBeginShape::{Close, NadaShape};
use libfinal::shape::{begin_shape, end_shape, stroke_weight, vertex};
use libfinal::transform::{rotate_z, translate};

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    cir_path: Vec<PVector3>,
    tri_path: Vec<PVector3>,
    mor_path: Vec<PVector3>,
    spacing: usize,
    theta: f32,
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);
        let cir_path = vec![];
        let tri_path = vec![];
        let mor_path = vec![];

        engine.param.ancho = ANCHO as f32;
        engine.param.alto = ALTO as f32;
        Sketch {
            engine,
            cir_path,
            tri_path,
            mor_path,
            spacing: 10,
            theta: 0.0,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }

    pub fn polar_to_cartesian(&mut self, r: f32, angle: f32) -> PVector3 {
        pvector3(r * cos_gr(angle), r * sin_gr(angle), 0.0)
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        angle_mode(Degrees, &mut self.engine.param);
        let radius: f32 = 100.0;
        let mut start_a: usize = 0;
        let mut end_a: usize = 120;
        let mut start = self.polar_to_cartesian(radius, start_a as f32);
        let mut end = self.polar_to_cartesian(radius, end_a as f32);
        for a in (start_a..360).step_by(self.spacing) {
            let cv = self.polar_to_cartesian(radius, a as f32);
            self.cir_path.push(cv);
            //self.cirPath = append(s.cirPath, cv);
            //s.morPath = append(s.morPath, cv)
            let amt = (a % 120) / (end_a as usize - start_a);
            let tv = vector3lerp(&start, &end, amt as f32);
            self.tri_path.push(tv);
            //self.tri_path = append(self.triPath, tv);

            if (a + self.spacing) % 120 == 0 {
                start_a = start_a + 120;
                end_a = end_a + 120;
                start = self.polar_to_cartesian(radius, start_a as f32);
                end = self.polar_to_cartesian(radius, end_a as f32);
            }
        }
        //        //createcanvas(&mut self.engine, ANCHO, ALTO);
        //        for j in 0..self.rows {
        //            for i in 0..self.cols {
        //                self.grid[j][i] = random_usize(2);
        //            }
        //        }
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        background(canvas, &mut self.engine, 220);

        translate(
            ANCHO as f32 / 2.0,
            ALTO as f32 / 2.0,
            &mut self.engine.param,
        );
        rotate_z(30.0, &mut self.engine.param);
        stroke1(0.0, &mut self.engine.param);
        stroke_weight(4, &mut self.engine.param);
        no_fill(&mut self.engine.param);
        let amt = (sin_gr(self.theta) + 1.0) / 2.0;
        self.theta += 5.0;

        begin_shape(NadaShape);
        for i in 0..self.cir_path.len() {
            let cv = self.cir_path[i];
            let tv = self.tri_path[i];

            let x = lerp(cv.x, tv.x, amt);
            let y = lerp(cv.y, tv.y, amt);
            vertex(x, y, &mut self.engine.param);
        }
        end_shape(canvas, &mut self.engine.param, Close);

        //P5Image::load_pixels(&mut self.engine.param, rl, &th);

        //        for i in 0..COLS - 1 {
        //            for j in 0..ROWS - 1 {
        //                self.current[i][j] = (self.previous[i - 1][j] +
        //                    self.previous[i + 1][j] +
        //                    self.previous[i][j - 1] +
        //                    self.previous[i][j + 1]) / 2 -
        //                    self.current[i][j];
        //
        //                self.current[i][j] = ((self.current[i][j]) as f32 * self.dampening) as u8;
        //
        //                // A diferencia de Processing, el arreglo píxeles en p5.js tiene 4 entradas
        //                // para cada píxel, por lo que tenemos que multiplicar el índice por 4 y luego
        //                // establecer las entradas para cada componente de color por separado.
        //                let index = (i + j * COLS) * 4;
        //                self.engine.param.pixels[index + 0] = self.current[i][j];
        //                self.engine.param.pixels[index + 1] = self.current[i][j];
        //                self.engine.param.pixels[index + 2] = self.current[i][j];
        //            }
        //        }
    }
}
