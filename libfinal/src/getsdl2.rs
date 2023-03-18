// Este modulo sirve para devolver elementos de sdl2 para la función que llama no tenga que
// importar librerías de sdl2
// zimage es el crate image renombrado para no confundirlo con sdl2::image

use std::fs::File;
use std::io::Read;

use sdl2::image;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum::ABGR8888;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use sdl2::image::LoadTexture;

use zimage::io::Reader as ImageReader;
use zimage::GenericImageView;

use crate::image::{PImage, pimage_vacio};

// Crea una Texture de ancho por alto
pub fn get_texture(ancho: u32, alto: u32, texture_creator: &mut TextureCreator<WindowContext>) -> Texture {
    texture_creator.create_texture_streaming(None, ancho, alto).unwrap()
}

// Crea una Texture de fichero png
pub fn get_texture_de_png(file: String, texture_creator: &mut TextureCreator<WindowContext>) -> Texture {
    texture_creator.load_texture(file).unwrap()
}

// Crea una Texture de un arreglo de u8
// pub fn get_texture_de_bytes(texture_creator: &mut TextureCreator<WindowContext>) -> Texture {
//     let mut file = File::open("resources/imagenes/papa.png").unwrap();
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer).unwrap();
//
//     for i in 100..200 {
//         buffer[i] = 0;
//     }
//     texture_creator.load_texture_bytes(&buffer).unwrap()
// }

// Crea una Surface de ancho por alto
pub fn get_surface(ancho: u32, alto: u32) -> Surface<'static> {
    Surface::new(ancho, alto, ABGR8888).unwrap()
}

// Da un punto sdl2 en x,y
pub fn get_punto(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

// Devuelve un Rec de sdl2
pub fn get_rect(x: i32, y: i32, ancho: u32, alto: u32) -> Rect {
    Rect::new(x, y, ancho, alto)
}

// Da un color sdl2 de rgb
pub fn get_color_rgb(r: u8, g: u8, b: u8) -> Color {
    Color::RGB(r, g, b)
}

//
pub fn cambia_color_surface(surface: &mut Surface) {
    // Cambiar el color de los píxeles en la posición (100, 100)
    let color = Color::RGB(255, 0, 0); // rojo
    surface.fill_rect(Some(Rect::new(10, 10, 200, 200)), color);
}

// Dibujar la textura en el canvas
pub fn dibuja_textura(canvas: &mut Canvas<Window>, x: i32, y: i32, ancho: u32, alto: u32, texture: &Texture) {
    // si src es None, se copia desde toda la textura
    canvas.copy(&texture, None, Some(Rect::new(x, y, ancho, alto))).unwrap();
}

// Devuelve un image_context
pub fn devuelve_image_context() -> Sdl2ImageContext {//} ImageContext {
    image::init(InitFlag::PNG | InitFlag::JPG).unwrap()
}

// Es una prueba, devuelve un Vec<u8> de un fichero
fn lee_file_a_vec(file: &str) -> Vec<u8> {
    let mut file = File::open(file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

