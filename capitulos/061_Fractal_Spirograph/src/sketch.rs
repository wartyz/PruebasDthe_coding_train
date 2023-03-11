use libfinal::color::{background, stroke3};
use libfinal::engine::{Canvas, Engine, Window};
use libfinal::environment::full_screen;
use libfinal::matem::{PVector3, pvector3};
use libfinal::parametros::ModosBeginShape;
use libfinal::shape::{begin_shape, end_shape, stroke_weight, vertex};
use crate::orbit::Orbit;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 600;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    path: Vec<PVector3>,
    //angle: f32,
    contador: usize,
    todos: Vec<Option<Orbit>>,

    resolution: f32,

    end: usize,
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
            path: vec![],
            contador: 0,
            todos: vec![],
            resolution: 10.,
            end: 0,
        }
    }
    pub fn pre_load(&mut self) {
        //full_screen(&mut self.engine);
    }
    // Función setup() de javascript
    pub fn setup(&mut self) {
        /* -----------------------------------------------------------*/
        // Crea sol
        let sun = Orbit::new3(300.0, 300.0, 150.0, 0, self.resolution, Some(0), None);
        // Mete el sol en el arrglo todos
        self.todos.push(Some(sun));
        /* -----------------------------------------------------------*/

        for i in 0..10 {
            self.contador += 1;

            // Le dice al padre que tiene un hijo
            let mut s = self.todos[i].unwrap();
            s.child = Some(self.contador);
            self.todos[i] = Some(s);

            // Crea el hijo
            let padre_de = self.todos[i].unwrap();
            let orb1 = padre_de.add_child(self.resolution, Some(self.contador));

            // Mete el hijo creado en el arreglo todos
            self.todos.push(Some(orb1));
        }

        self.end = self.contador;
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

        self.todos[0].unwrap().show(&mut self.engine, canvas);

        let mut next = Some(0);
        while next.is_some() {
            let indice = next.unwrap();
            //dbg!(indice);

            let indice_padre = self.todos[indice].unwrap().parent;
            let mut padre = None;
            if indice_padre.is_none() {
                padre = None;
            } else {
                padre = self.todos[indice_padre.unwrap()];
            }

            // Ojo al hacer unwrap() sacamos una copia, hay que volverla a meter con Some()
            self.todos[indice] = Some(self.todos[indice].unwrap().update(padre));

            self.todos[indice].unwrap().show(&mut self.engine, canvas);
            next = self.todos[indice].unwrap().child;
            //dbg!(next);
        }

        let obj_end = self.todos[self.end].unwrap();
        self.path.push(pvector3(obj_end.x, obj_end.y, 1.0));
        //}

        begin_shape(ModosBeginShape::NadaShape);
        stroke3(255.0, 0.0, 255.0, &mut self.engine.param);
        stroke_weight(2, &mut self.engine.param);
        for pos in &self.path {
            vertex(pos.x, pos.y, &mut self.engine.param);
        }
        end_shape(canvas, &mut self.engine.param, ModosBeginShape::NadaShape);
    }
}
