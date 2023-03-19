/*


    PGraphics

    Main graphics and rendering context, as well as the base API implementation for processing "core"
    blendMode()

    Blends the pixels in the display window according to a defined mode
    clip()

    Limits the rendering to the boundaries of a rectangle defined by the parameters
    createGraphics()

    Creates and returns a new PGraphics object of the types P2D or P3D
    hint()

    This function is used to enable or disable special features that control how graphics are drawn
    noClip()

    Disables the clipping previously started by the clip() function

Shaders

    PShader

    This class encapsulates a GLSL shader program, including a vertex and a fragment shader
    loadShader()

    Loads a shader into the PShader object
    resetShader()

    Restores the default shaders
    shader()

    Applies the shader specified by the parameters


 */

use sdl2::pixels::PixelFormatEnum::ARGB8888;
use sdl2::rect::Rect;
pub use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::render::BlendMode;
use sdl2::video::Window;
use crate::parametros::Parametros;
use crate::shape::line;

pub struct PGraphics {
    // Vector que crece con los puntos que recibe
    coord: Vec<(f32, f32, f32, f32)>,
}

impl PGraphics {
    /// Establece las propiedades predeterminadas para un objeto PGraphics
    pub fn begin_draw(&self) {}

    /// Finaliza el renderizado de un objeto PGraphics para que pueda mostrarse en la pantalla
    pub fn end_draw(&self) {}

    pub fn background(&self) {
        //background()
    }
}

pub fn blend_mode() { unimplemented!(); }

pub fn clip() { unimplemented!(); }

pub fn create_graphics() -> PGraphics {
    let coord = vec![];

    PGraphics {
        coord,
    }
}

pub fn hint() { unimplemented!(); }

pub fn no_clip() { unimplemented!(); }

// Shaders *************************

// PShader

pub fn load_shader() { unimplemented!(); }

pub fn reset_shader() { unimplemented!(); }

pub fn shader() { unimplemented!(); }

// Funciones creadas por mi
impl PGraphics {
    pub fn set_points(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        self.coord.push((x0, y0, x1, y1));
    }

    /// Dibuja las lineas con los puntos que estan en self.coord
    pub fn presenta_pixels(&self, canvas: &mut Canvas<Window>, param: &mut Parametros) {
        for p in &self.coord {
            line(canvas, param, p.0, p.1, p.2, p.3);
        }
    }
}

