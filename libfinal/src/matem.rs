/*
Math
    PVector
Calculation
    abs()
    ceil()
    constrain()
    dist()
    exp()
    floor()
    lerp()
    log()
    mag()
    map()
    max()
    min()
    norm()
    pow()
    round()
    sq()
    sqrt()
Trigonometry
    acos()
    asin()
    atan2()
    atan()
    cos()
    degrees()
    radians()
    sin()
    tan()
Operators
    += (add assign)
    + (addition)
    -- (decrement)
    / (divide)
    /= (divide assign)
    ++ (increment)
    - (minus)
    % (modulo)
    * (multiply)
    *= (multiply assign)
    -= (subtract assign)
Bitwise Operators
    & (bitwise AND)
    | (bitwise OR)
    << (left shift)
    >> (right shift)
Random
    noiseDetail()
    noiseSeed()
    noise()
    randomGaussian()
    randomSeed()
    random()
 */
use rand::Rng;

// Math *******************************
#[derive(Debug, Clone)]
pub struct PVector2 {
    pub x: f32,
    pub y: f32,

}

impl Default for PVector2 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,

        }
    }
}

#[derive(Debug, Clone)]
pub struct PVector3 {
    pub x: f32,
    pub y: f32,
    pub w: f32,
}

impl Default for PVector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PVector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for PVector4 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

// Calculation *****************************
pub fn abs() { unimplemented!(); }

pub fn ceil() { unimplemented!(); }

pub fn constrain() { unimplemented!(); }

pub fn dist() { unimplemented!(); }

pub fn dist4(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let x = x0 - x1;
    let y = y0 - y1;
    ((x * x) + (y * y)).sqrt()
}

// Distancia entre dos vectores  d = ((x2 - x1)2 + (y2 - y1)2 + (z2 - z1)2)1/2
pub fn dist_s(a: &PVector4, b: &PVector4) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;
    ((x * x) + (y * y) + (z * z)).sqrt()
}

pub fn exp() { unimplemented!(); }

pub fn floor() { unimplemented!(); }

pub fn lerp() { unimplemented!(); }

pub fn log() { unimplemented!(); }

pub fn mag() { unimplemented!(); }

// se llama map en jS, le cambio el nombre para que no hayan malentendidos
pub fn mapa(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    let fnn = n;
    let fstart1 = start1;
    let fstop1 = stop1;
    let fstart2 = start2;
    let fstop2 = stop2;

    ((fnn - fstart1) / (fstop1 - fstart1)) * (fstop2 - fstart2) + fstart2
}

pub fn max() { unimplemented!(); }

pub fn min() { unimplemented!(); }

pub fn norm() { unimplemented!(); }

pub fn pow() { unimplemented!(); }

pub fn round() { unimplemented!(); }

pub fn sq() { unimplemented!(); }

pub fn sqrt() { unimplemented!(); }

// Trigonometry *****************************
pub fn acos() { unimplemented!(); }

pub fn asin() { unimplemented!(); }

pub fn atan2() { unimplemented!(); }

pub fn atan() { unimplemented!(); }

pub fn cos() { unimplemented!(); }

pub fn degrees() { unimplemented!(); }

pub fn radians() { unimplemented!(); }

pub fn sin() { unimplemented!(); }

pub fn tan() { unimplemented!(); }

// Operators **********************************
// += (add assign)
// + (addition)
// -- (decrement)
// / (divide)
// /= (divide assign)
// ++ (increment)
// - (minus)
// % (modulo)
// * (multiply)
// *= (multiply assign)
// -= (subtract assign)
// Bitwise Operators *****************************
// & (bitwise AND)
// | (bitwise OR)
// << (left shift)
// >> (right shift)
// Random *******************************
pub fn noise_detail() { unimplemented!(); }

pub fn noise_seed() { unimplemented!(); }

pub fn noise() { unimplemented!(); }

pub fn random_gaussian() { unimplemented!(); }

pub fn random_seed() { unimplemented!(); }

pub fn random(p0: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, p0)
}

// Funciones creadas por mi ********************************

pub fn pvector2(x: f32, y: f32) -> PVector2 {
    PVector2 { x, y }
}

pub fn pvector3(x: f32, y: f32, w: f32) -> PVector3 {
    PVector3 { x, y, w }
}

pub fn pvector4(x: f32, y: f32, z: f32, w: f32) -> PVector4 {
    PVector4 { x, y, z, w }
}

pub fn random_range(p0: f32, p1: f32) -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(p0, p1)
}

pub fn random_i32_range(p0: i32, p1: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(p0, p1)
}

pub fn random_i32(p0: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, p0)
}

pub fn random_usize(p0: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, p0)
}
