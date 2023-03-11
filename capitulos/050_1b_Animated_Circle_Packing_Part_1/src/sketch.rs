use libfinal::color::{background, brightness, color4, PColor};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::image::{PImage, pimage_vacio};
use libfinal::matem::{dist_s, PVector3, pvector3, random_usize};
use crate::circle::Circle;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 900;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    circles: Vec<Circle>,
    img: Option<PImage>,
    spots: Vec<PVector3>,
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
            circles: vec![],
            //img: PImage::load_image("../resources/imagenes/2017.png"),
            img: None,
            spots: vec![],
        }
    }
    pub fn pre_load(&mut self) {
        full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self, canvas: &mut Canvas<Window>) {
        //createcanvas(&mut self.engine, 900, 400);

        // Carga una imagen en una variable de tipo PImage.
        let mut imagen = PImage::load_image(canvas, "resources/imagenes/2017_32.bmp").unwrap();

        //self.img = PImage::load_image(canvas, "../resources/imagenes/2017.png");
        // Carga los datos de píxeles de la ventana en el arreglo self.imagen.
        //imagen.load_pixels(canvas);

        // for x in (0..imagen.image_width as usize * 4).step_by(4) {
        //     for y in 0..imagen.image_height as usize {

        /*

        for x in 0..image_width as usize {
        let mut vv = vec![];
        for y in 0..image_height as usize {
            let index = (y * image_width as usize + x) * 4;

            let valor = (
                pixels[index],
                pixels[index + 1],
                pixels[index + 2],
                pixels[index + 3],
            );

            vv.push(valor);
        }

        image.push(vv);
    }
         */
        for x in 0..imagen.image_width as usize {
            //dbg!(self.image[y].len());
            //for x in 0..self.image[y].len() {
            for y in 0..imagen.image_height as usize {
                //println!(" p = {:?}", self.img.image_pixels[y as usize]);
                //let index = (y * imagen.image_width as usize + x) * 4;
                // buffer[index] = self.image[x][y].0; // B
                let r = imagen.image[x][y].0;
                let g = imagen.image[x][y].1;
                let b = imagen.image[x][y].2;
                let a = imagen.image[x][y].3;

                // dbg!(r);
                // dbg!(g);
                // dbg!(b);
                // dbg!(a);

                let br = brightness(color4(r, g, b, a));

                if br > 1.0 {
                    //self.spots.push(pvector3(x as f32 / 4.0, y as f32, 1.0));
                    self.spots.push(pvector3(x as f32, y as f32, 1.0));
                }
            }
        }

        //println!("spots.len() = {:?}", self.spots.len());

        self.circles.push(Circle::new(200.0, 200.0));
        //        for i in 0..(self.max_stars) {
        //            self.stars.push(Star::new(&mut self.engine.param));
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
        background(canvas, &mut self.engine, 0);

        let total = 10;
        let mut count = 0;
        let mut attempts = 0;

        while count < total {
            let new_c = self.new_circle();

            if new_c.is_some() {
                self.circles.push(new_c.unwrap());
                count += 1;
            }
            attempts += 1;
            if attempts > 1000 {
                println!("FINISHED");
                break;
            }
        }

        for i in 0..self.circles.len() {
            if self.circles[i].growing {
                if self.circles[i].edges(&mut self.engine.param) {
                    self.circles[i].growing = false;
                } else {
                    for other in &self.circles {
                        if self.circles[i] != *other {
                            let d = dist_s(
                                &pvector3(self.circles[i].x, self.circles[i].y, 1.0),
                                &pvector3(other.x, other.y, 1.0),
                            );
                            if d < self.circles[i].r + other.r {
                                self.circles[i].growing = false;
                                break;
                            }
                        }
                    }
                }
            }
            self.circles[i].show(&mut self.engine, canvas);
            self.circles[i].grow();
        }
    }

    // Función del sketch
    pub fn new_circle(&mut self) -> Option<Circle> {
        let r = random_usize(self.spots.len());
        let spot = self.spots.get(r).unwrap();

        let x = spot.x;
        let y = spot.y;

        let mut valid = true;
        for c in &self.circles {
            let d = dist_s(&pvector3(x, y, 1.0), &pvector3(c.x, c.y, 1.0));
            if d < c.r + 2.0 {
                valid = false;
                break;
            }
        }

        if valid {
            return Some(Circle::new(x, y));
        }
        None
    }
}
