// Prueba de https://github.com/pixlark/ColorPicker/blob/master/main.c

use crate::color::{color4, PColor};
use crate::matem::mapa;

// Vienen valore       0-360    0-100   0-100
pub fn aux_hsv_to_rgb2(inh: f32, ins: f32, inb: f32, ina: f32) -> PColor {
    //RGBColor hsv_to_rgb(HSVColor in)

    //    let inh = mapa(h as f32, 0.0, 255.0, 0.0, 360.0);
    //    //let inh = h as f32;
    //    let ins = mapa(s as f32, 0.0, 255.0, 0.0, 1.0);
    //    let inb = mapa(b as f32, 0.0, 255.0, 0.0, 1.0);

    //let inh = h as f32;
    //let ins = s as f32;
    //let inb = b as f32;

    let mut hh;
    let p;
    let q;
    let t;
    let ff;
    let i;
    let mut out = color4(0, 0, 0, 0);

    if ins <= 0.0 {
        // < is bogus, just shuts up warnings
        out.r = inb as u8;
        out.g = inb as u8;
        out.b = inb as u8;
        return out;
    }
    hh = inh;
    if hh >= 360.0 {
        hh = 0.0;
    }
    hh /= 60.0;
    i = hh as i32;
    ff = hh - i as f32;
    p = inb * (1.0 - ins);
    q = inb * (1.0 - (ins * ff));
    t = inb * (1.0 - (ins * (1.0 - ff)));
    //println!("0 en utiles::aux_hsv_to_rgb2 i = {:?}", i);
    match i {
        0 => {
            out.r = inb as u8;
            out.g = t as u8;
            out.b = p as u8;
        }
        1 => {
            out.r = q as u8;
            out.g = inb as u8;
            out.b = p as u8;
        }
        2 => {
            out.r = p as u8;
            out.g = inb as u8;
            out.b = t as u8;
        }
        3 => {
            out.r = p as u8;
            out.g = q as u8;
            out.b = inb as u8;
        }

        4 => {
            out.r = t as u8;
            out.g = p as u8;
            out.b = inb as u8;
        }
        5 => {
            out.r = inb as u8;
            out.g = p as u8;
            out.b = q as u8;
        }
        _ => {
            out.r = inb as u8;
            out.g = p as u8;
            out.b = q as u8;
        }
    }
    out.a = ina as u8;

    out.r = mapa(out.r as f32, 0.0, 0.1, 0.0, 255.0) as u8;
    out.g = mapa(out.g as f32, 0.0, 0.1, 0.0, 255.0) as u8;
    out.b = mapa(out.b as f32, 0.0, 0.1, 0.0, 255.0) as u8;

    //println!("en utiles::aux_hsv_to_rgb2 out = {:?}", out);
    out
}