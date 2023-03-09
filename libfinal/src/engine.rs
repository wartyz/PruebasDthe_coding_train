use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
pub use sdl2::render::Canvas;
pub use sdl2::video::Window;
use sdl2::Sdl;
use crate::parametros::*;
use crate::transform::identity3x3;

use test;

pub struct Engine {
    pub sdl_context: Option<Sdl>,
    pub param: Parametros,
    pub parambakup: Parametros,
    pub si_ventana: bool,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(100.0, 200.0)
    }
}

impl Engine {
    pub fn new(ancho: f32, alto: f32) -> Engine {
        Engine {
            sdl_context: None,
            param: Parametros::new(ancho, alto),
            parambakup: Parametros::new(ancho, alto),
            si_ventana: false,
        }
    }

    pub fn update(&mut self) -> bool {
        self.reinicia_matrices();
        //self.reinicia_vertex();
        self.reinicia_teclado();
        self.reinicia_raton();
        // Crea contexto SDL2
        let sdl_context = self.sdl_context.as_ref().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        // Eventos
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                Event::MouseButtonDown {
                    x: _,
                    y: _,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    println!("MouseButtonDown");

                    self.param.mouse_boton_mantiene = CodigosRaton::Izquierdo;
                }
                Event::MouseMotion { x, y, .. } => {
                    self.param.mouse_posicion.x = x as f32;
                    self.param.mouse_posicion.y = y as f32;
                }

                Event::KeyUp {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => match (keycode, keymod) {
                    (Keycode::A, _) => {
                        self.param.keyr = CodigosTecla::A;
                    }
                    (Keycode::J, _) => {
                        self.param.keyr = CodigosTecla::J;
                    }
                    (Keycode::M, _) => {
                        self.param.keyr = CodigosTecla::M;
                    }
                    (Keycode::Z, _) => {
                        self.param.keyr = CodigosTecla::Z;
                    }
                    _ => (),
                },
                Event::KeyDown {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => match (keycode, keymod) {
                    (Keycode::A, _) => {
                        self.param.key = CodigosTecla::A;
                    }
                    (Keycode::J, _) => {
                        self.param.key = CodigosTecla::J;
                    }
                    (Keycode::M, _) => {
                        self.param.key = CodigosTecla::M;
                    }
                    (Keycode::Z, _) => {
                        self.param.key = CodigosTecla::Z;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        true
    }

    pub fn reinicia_matrices(&mut self) {
        self.param.matriz_total = identity3x3();
    }

    /*    // Aqui se reinicia en arreglo vertex: Vec<Vector2> para shapes
        pub fn reinicia_vertex(&mut self) {
            self.param.vertex.clear();
        }*/

    // Aqui se reinicia en el indicador de tecla presionad
    pub fn reinicia_teclado(&mut self) {
        self.param.key = CodigosTecla::NadaTecla;
        self.param.keyr = CodigosTecla::NadaTecla;
    }

    // Aqui se reinicia en el indicador de boton de raton presionado
    pub fn reinicia_raton(&mut self) {
        self.param.mouse_boton_inicio = CodigosRaton::NadaRaton;
        self.param.mouse_boton_mantiene = CodigosRaton::NadaRaton;
        self.param.mouse_boton_final = CodigosRaton::NadaRaton;
    }
}
