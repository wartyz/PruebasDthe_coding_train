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

pub use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use crate::color::background;

pub struct PGraphics {
    ancho: u32,
    alto: u32,
    pixels: Vec<u32>,

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

pub fn create_graphics<'a>(ancho: u32, alto: u32) -> PGraphics {
    let mut pixels = Vec::with_capacity((ancho * alto) as usize);
    for _ in 0..(ancho * alto) {
        pixels.push(0);
    }

    // let texture = texture_creator.create_texture_streaming(
    //     sdl2::pixels::PixelFormatEnum::ABGR8888,
    //     ancho,
    //     alto,
    // );

    PGraphics {
        ancho,
        alto,
        pixels,

    }
}

pub fn hint() { unimplemented!(); }

pub fn no_clip() { unimplemented!(); }

// Shaders *************************

// PShader

pub fn load_shader() { unimplemented!(); }

pub fn reset_shader() { unimplemented!(); }

pub fn shader() { unimplemented!(); }

