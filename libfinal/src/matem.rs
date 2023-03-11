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

#[derive(Debug, Clone, Copy)]
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

// constrain(n, low, high) Restringe un valor entre un valor mínimo y máximo.
pub fn constrain(n: f32, low: f32, high: f32) -> f32 {
    if n < low {
        return low;
    }
    if n > high {
        return high;
    }
    n
}

pub fn dist() { unimplemented!(); }

pub fn dist4(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let x = x0 - x1;
    let y = y0 - y1;
    ((x * x) + (y * y)).sqrt()
}

pub fn exp() { unimplemented!(); }

pub fn floor(v: f32) -> f32 {
    v.floor()
}

// Interpolación lineal https://en.wikipedia.org/wiki/Linear_interpolation
pub fn lerp(start: f32, stop: f32, amt: f32) -> f32 {
    //if amt > 1.0 || amt < 0.0 {
    if !(0.0..=1.0).contains(&amt) {
        panic!("Error en matem::lerp");
    }
    (1.0 - amt) * start + amt * stop
}

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

// Funciones creadas por mi ******** algunas estaban en la librería P5 ************************

impl PVector3 {
    // Normaliza este vector
    pub fn normalize(&mut self) {
        let a = self.x * self.x;
        let b = self.y * self.y;
        //let c = float64(v.Z * v.Z)

        let longitud = (a + b).sqrt();

        if longitud != 0.0 {
            self.x /= longitud;
            self.y /= longitud;
            //v.Z /= longitud
        }
    }

    // Multiplica este vector por un escalar
    pub fn mult(&mut self, v: f32) {
        self.x *= v;
        self.y *= v;
    }

    // Suma a este otro vector
    pub fn add(&mut self, b: &PVector3) {
        self.x += b.x;
        self.y += b.y;
        self.w += b.w;
    }

    // Divide este vector por un numero
    pub fn div(&mut self, b: f32) {
        self.x /= b;
        self.y /= b;
    }

    // Limita la magnitud máxima del vector
    pub fn limit(&mut self, max: f32) {
        let magnitud_actual = (self.x * self.x + self.y * self.y).sqrt();
        if magnitud_actual > max {
            self.set_mag(max);
        }
    }

    // Devuelve la magnitud de este vector
    pub fn mag(&mut self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Devuelve la magnitud cuadrada de este vector
    pub fn mag_sq(&mut self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    // Establece la magnitud de este vector
    pub fn set_mag(&mut self, magnitud: f32) {
        self.normalize();
        self.x *= magnitud;
        self.y *= magnitud;
    }

    pub fn lerp(&mut self, vector: PVector3, amt: f32) {
        self.x = (1.0 - amt) * self.x + amt * vector.x;
        self.y = (1.0 - amt) * self.y + amt * vector.y;
    }
}

pub fn pvector2(x: f32, y: f32) -> PVector2 {
    PVector2 { x, y }
}

pub fn pvector3(x: f32, y: f32, w: f32) -> PVector3 {
    PVector3 { x, y, w }
}

pub fn pvector4(x: f32, y: f32, z: f32, w: f32) -> PVector4 {
    PVector4 { x, y, z, w }
}

// Distancia entre dos pvectores
pub fn dist_s(a: &PVector3, b: &PVector3) -> f32 {
    //let pa = Point2::new(a.x, a.y);
    //let pb = Point2::new(b.x, b.y);

    let x = a.x - b.x;
    let y = a.y - b.y;
    ((x * x) + (y * y)).sqrt()
}

// Distancia entre dos pvectores3
pub fn dist_s3(a: &PVector3, b: &PVector3) -> f32 {
    //let pa = Point2::new(a.x, a.y);
    //let pb = Point2::new(b.x, b.y);

    let x = a.x - b.x;
    let y = a.y - b.y;
    ((x * x) + (y * y)).sqrt()
}

// Distancia entre dos pvectores4  d = ((x2 - x1)2 + (y2 - y1)2 + (z2 - z1)2)1/2
pub fn dist_s4(a: &PVector4, b: &PVector4) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;
    ((x * x) + (y * y) + (z * z)).sqrt()
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

// Devuelve un PVector3 con valores aleatórios
pub fn random2d() -> PVector3 {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-1.0, 1.0);
    let y: f32 = rng.gen_range(-1.0, 1.0);
    //println!("En p5_vector random2d x= {:?}", x);
    let mut v = pvector3(x, y, 0.0);
    v.normalize();
    v
}

