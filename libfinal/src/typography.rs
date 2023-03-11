/*



    PFont

    Grayscale bitmap font class used by Processing

Loading & Displaying

    createFont()

    Dynamically converts a font to the format used by Processing
    loadFont()

    Loads a font into a variable of type PFont
    textFont()

    Sets the current font that will be drawn with the text() function
    text()

    Draws text to the screen

Attributes

    textAlign()

    Sets the current alignment for drawing text
    textLeading()

    Sets the spacing between lines of text in units of pixels
    textMode()

    Sets the way text draws to the screen
    textSize()

    Sets the current font size
    textWidth()

    Calculates and returns the width of any character or text string

Metrics

    textAscent()

    Returns ascent of the current font at its current size
    textDescent()

    Returns descent of the current font at its current size


 */
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::FontStyle;
use sdl2::video::{Window, WindowContext};
use crate::engine::Engine;
use crate::parametros::Parametros;
// PFont

// **** Loading & Displaying

pub fn create_font() { unimplemented!(); }

pub fn load_font() { unimplemented!(); }

pub fn text_font() { unimplemented!(); }

pub fn text(engine: &mut Engine, canvas: &mut Canvas<Window>, txt: &str, x: f32, y: f32) {
    //let canvas = engine.canvas.as_mut().unwrap();
    let ttf_context = sdl2::ttf::init().expect("Fallo en la inicialización de SDL TTF");

    // Cargamos la font
    let mut font = ttf_context
        .load_font(
            "resources/fonts/Lucida Sans.ttf",
            engine.param.tamafont as u16,
        )
        .expect("No se puede cargar la font");

    // Le aplicamos un estilo
    font.set_style(FontStyle::BOLD);

    // Anchura y altura del texto entero
    let anchura: u32 = 20;
    let altura: u32 = 30;

    // Obtenemos un creador de textura
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // Creamos una textura del texto
    let rendered_text = create_texture_from_text(&texture_creator, &font, txt, 255, 255, 255)
        .expect("No se puede renderizar texto");

    // Se pasa la textura a la pantalla
    canvas
        .copy(
            &rendered_text,
            None,
            Some(Rect::new(x as i32, y as i32, anchura, altura)),
        )
        .expect("No se puede copiar texto a la pantalla");
}

// ****** Attributes

pub fn text_align() { unimplemented!(); }

pub fn text_leading() { unimplemented!(); }

pub fn text_mode() { unimplemented!(); }

pub fn text_size(param: &mut Parametros, t: i32) {
    param.tamafont = t;
}

pub fn text_width() { unimplemented!(); }

// ******* Metrics

pub fn text_ascent() { unimplemented!(); }

pub fn text_descent() { unimplemented!(); }

// ****** Funciones creadas por mi *****************************************
fn create_texture_from_text<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    text: &str,
    r: u8,
    g: u8,
    b: u8,
) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text).blended(Color::RGB(r, g, b)) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

