/*
Creating & Reading
    alpha()
    blue()
    brightness()
    color()
    green()
    hue()
    lerpColor()
    red()
    saturation()

Setting
    background()
    clear()
    colorMode()
    fill()
    noFill()
    noStroke()
    stroke()
*/
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::engine::Engine;
use crate::parametros::{ColorMode, Parametros};
use crate::utiles::aux_hsv_to_rgb2;

#[derive(Debug, Clone, Copy)]
pub struct PColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl PColor {
    fn new4(r: u8, g: u8, b: u8, a: u8) -> PColor {
        PColor { r, g, b, a }
    }

    fn new3(r: u8, g: u8, b: u8) -> PColor {
        PColor { r, g, b, a: 255 }
    }
}

// **************** Creating & Reading ************************
pub fn alpha() { unimplemented!(); }

pub fn blue() { unimplemented!(); }

pub fn brightness() { unimplemented!(); }

pub fn color1(n: u8) -> PColor {
    PColor::new3(n, n, n)
}

pub fn color3(r: u8, g: u8, b: u8) -> PColor {
    PColor::new3(r, g, b)
}

pub fn color4(r: u8, g: u8, b: u8, a: u8) -> PColor {
    PColor::new4(r, g, b, a)
}

pub fn green() { unimplemented!(); }

pub fn hue() { unimplemented!(); }

pub fn lerp_color() { unimplemented!(); }

pub fn red() { unimplemented!(); }

pub fn saturation() { unimplemented!(); }

// **************** Setting ****************************

pub fn background(canvas: &mut Canvas<Window>, engine: &mut Engine, c: u8) {
    let color = color4(c, c, c, 255);
    engine.param.color_background = color;
    //engine.canvas.unwrap().clear();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(color.r, color.g, color.b));
    canvas.clear();
}

pub fn clear() { unimplemented!(); }

pub fn color_mode() { unimplemented!(); }

pub fn fill(color: PColor, param: &mut Parametros) {
    param.fill_color = color;
}

pub fn fill1(c: f32, param: &mut Parametros) {
    param.fill_color = pcolor4(c as u8, c as u8, c as u8, 255);
}

pub fn fill2(c: f32, a: f32, param: &mut Parametros) {
    param.fill_color = pcolor4(c as u8, c as u8, c as u8, a as u8);
}

pub fn fill3(r: f32, g: f32, b: f32, param: &mut Parametros) {
    param.fill_color = match param.colormode {
        ColorMode::RGB => pcolor4(r as u8, g as u8, b as u8, 255),
        ColorMode::HSB => aux_hsv_to_rgb2(r, g, b, 255.0),
        _ => panic!("Error"),
    };

    //    let mut col;
    //    if param.colormode == ColorMode::RGB {
    //        col = Color::new(r as u8, g as u8, b as u8, 255);
    //    }
    //    if param.colormode == ColorMode::HSB {
    //
    //    }
    //param.fill_color = col;
}

pub fn fill4(r: f32, g: f32, b: f32, a: f32, param: &mut Parametros) {
    param.fill_color = pcolor4(r as u8, g as u8, b as u8, a as u8);
}

pub fn no_fill() { unimplemented!(); }

pub fn no_stroke(param: &mut Parametros) {
    param.stroke_bool = false;
}

pub fn stroke1(v: f32, param: &mut Parametros) {
    param.stroke_color = match param.colormode {
        ColorMode::RGB => color4(v as u8, v as u8, v as u8, 255),
        ColorMode::HSB => aux_hsv_to_rgb2(v, v, v, 255.0),
        _ => panic!("Error"),
    };
}

// Recibe color y transparencia
pub fn stroke2(c: f32, a: f32, param: &mut Parametros) {
    param.stroke_color = match param.colormode {
        ColorMode::RGB => color4(c as u8, c as u8, c as u8, a as u8),
        ColorMode::HSB => aux_hsv_to_rgb2(c, c, c, a),
        _ => panic!("Error"),
    };
}

pub fn stroke3(r: f32, g: f32, b: f32, param: &mut Parametros) {
    param.stroke_color = match param.colormode {
        ColorMode::RGB => color4(r as u8, g as u8, b as u8, 255),
        ColorMode::HSB => aux_hsv_to_rgb2(r, g, b, 255.0),
        _ => panic!("Error"),
    };
}

// ** Funciones añadidas por mi *****************
pub fn pcolor4(r: u8, g: u8, b: u8, a: u8) -> PColor {
    PColor::new4(r, g, b, a)
}