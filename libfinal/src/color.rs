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
pub fn background() { unimplemented!(); }

pub fn clear() { unimplemented!(); }

pub fn color_mode() { unimplemented!(); }

pub fn fill() { unimplemented!(); }

pub fn no_fill() { unimplemented!(); }

pub fn no_stroke() { unimplemented!(); }

pub fn stroke() { unimplemented!(); }