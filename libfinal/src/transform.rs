/*
    applyMatrix()
    popMatrix()
    printMatrix()
    pushMatrix()
    resetMatrix()
    rotateX()
    rotateY()
    rotateZ()
    rotate()
    scale()
    shearX()
    shearY()
    translate()

 */



use std::ops::Mul;
use crate::matem::{PVector3, pvector3};
use crate::parametros::Parametros;

pub fn apply_matrix() { unimplemented!(); }

pub fn pop_matrix(param: &mut Parametros) {
    param.matriz_total = param.matriz_stack.pop().unwrap();
}

pub fn print_matrix() { unimplemented!(); }

// Pone la matriz de transformación en la pila de matriz.
pub fn push_matrix(param: &mut Parametros) {
    param.matriz_stack.push(param.matriz_total);
}

pub fn reset_matrix() { unimplemented!(); }

// carga la matriz de rotacion del eje x con una coordenada
#[rustfmt::skip]
pub fn rotate_x(angulo: f32, param: &mut Parametros) {
    let m = Matrix3x3 {
        data: [1.0, 0.0, 0.0,
            0.0, angulo.cos(), -angulo.sin(),
            0.0, angulo.sin(), angulo.cos(), ]
    };

    //param.matriz_rotacionx = UnitQuaternion::from_scaled_axis(Vector3::x() * angulo);
    param.matriz_total = param.matriz_total * m;
}

// carga la matriz de rotacion del eje y con una coordenada
#[rustfmt::skip]
pub fn rotate_y(angulo: f32, param: &mut Parametros) {
    let m = Matrix3x3 {
        data: [angulo.cos(), 0.0, angulo.sin(),
            0.0, 1.0, 0.0,
            -angulo.sin(), 0.0, angulo.cos(), ]
    };

    //param.matriz_rotaciony = UnitQuaternion::from_scaled_axis(Vector3::y() * angulo);
    param.matriz_total = param.matriz_total * m;
}

// Recibe un ángulo en radianes y carga la matriz de rotacion del eje z
#[rustfmt::skip]
pub fn rotate_z(angulo: f32, param: &mut Parametros) {
    let m = Matrix3x3 {
        data: [angulo.cos(), -angulo.sin(), 0.0,
            angulo.sin(), angulo.cos(), 0.0,
            0.0, 0.0, 1.0]
    };

    param.matriz_total = param.matriz_total * m;
}

pub fn rotate() { unimplemented!(); }

pub fn scale() { unimplemented!(); }

pub fn shear_x() { unimplemented!(); }

pub fn shear_y() { unimplemented!(); }

// Translación respecto a los ejes globales
#[rustfmt::skip]
pub fn translate(x: f32, y: f32, param: &mut Parametros) {
    let m = Matrix3x3 {
        data: [1.0, 0.0, x,
            0.0, 1.0, y,
            0.0, 0.0, 1.0]
    };

    param.matriz_total = param.matriz_total * m;
}

// *************************  Funciones creadas por mi ***********************
// USO VECTOR COLUMNA
// Ojo para el cálculo la matriz es 0  1  2    y el vector  x
//                                  3  4  5                 y
//
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2x2 {
    pub data: [f32; 4],
}

impl Matrix2x2 {
    #[rustfmt::skip]
    pub fn new(
        m00: f32, m01: f32,
        m02: f32, m03: f32,
    ) -> Self {
        Matrix2x2 {
            data: [m00, m01,
                m02, m03, ]
        }
    }

    pub fn determinant(&self) -> f32 {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix3x3 {
    pub data: [f32; 9],
}

impl Matrix3x3 {
    #[rustfmt::skip]
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m03: f32, m04: f32, m05: f32,
        m06: f32, m07: f32, m08: f32,
    ) -> Self {
        Matrix3x3 {
            data: [m00, m01, m02,
                m03, m04, m05,
                m06, m07, m08, ]
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix2x2 {
        let mut matrix2x2 = identity2x2();
        let mut source_row: usize = 0;
        let mut source_column: usize = 0;
        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < 2 {
            if source_row == row {
                // Skip row to be removed
                source_row += 1;
            }
            while target_column < 2 {
                if source_column == column {
                    // Skip column to be removed
                    source_column += 1;
                }
                matrix2x2.data[target_column + target_row * 2] =
                    self.data[source_column + source_row * 3];

                source_column += 1;
                target_column += 1;
            }
            source_row += 1;
            source_column = 0;
            target_row += 1;
            target_column = 0;
        }
        matrix2x2
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            // Even value
            minor
        } else {
            -minor
        }
    }
    pub fn determinant(&self) -> f32 {
        let mut determinant = 0.0;
        for column in 0..3 {
            determinant += self.cofactor(0, column) * self.data[column];
        }

        determinant
    }
}

#[rustfmt::skip]
pub fn identity2x2() -> Matrix2x2 {
    Matrix2x2::new(1.0, 0.0,
                   0.0, 1.0, )
}

#[rustfmt::skip]
pub fn identity3x3() -> Matrix3x3 {
    Matrix3x3 {
        data: [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0, ]
    }
}

// Escala igual en los dos ejes
#[rustfmt::skip]
pub fn scale1(v: f32, param: &mut Parametros) {
    let m = Matrix3x3 {
        data: [v, 0.0, 0.0,
            0.0, v, 0.0,
            0.0, 0.0, 1.0],
    };

    param.matriz_total = m * param.matriz_total;
}

// // Recibe dos matrices 3x3 y devuelve su producto  A x B    Ojo con el orden
// pub fn product_matrix_axb(a: &Matrix3x3, b: &Matrix3x3) -> Matrix3x3 {
//     let mut m = identity3x3();
//
//     let a00 = a.data[0];
//     let a01 = a.data[1];
//     let a02 = a.data[2];
//     let a10 = a.data[3];
//     let a11 = a.data[4];
//     let a12 = a.data[5];
//     let a20 = a.data[6];
//     let a21 = a.data[7];
//     let a22 = a.data[8];
//
//     let b00 = b.data[0];
//     let b01 = b.data[1];
//     let b02 = b.data[2];
//     let b10 = b.data[3];
//     let b11 = b.data[4];
//     let b12 = b.data[5];
//     let b20 = b.data[6];
//     let b21 = b.data[7];
//     let b22 = b.data[8];
//
// //    m.data[0] = b00 * a00 + b01 * a10 + b02 * a20;
// //    m.data[1] = b00 * a01 + b01 * a11 + b02 * a21;
// //    m.data[2] = b00 * a02 + b01 * a12 + b02 * a22;
// //
// //    m.data[3] = b10 * a00 + b11 * a10 + b12 * a20;
// //    m.data[4] = b10 * a01 + b11 * a11 + b12 * a21;
// //    m.data[5] = b10 * a02 + b11 * a12 + b12 * a22;
// //
// //    m.data[6] = b20 * a00 + b21 * a10 + b22 * a20;
// //    m.data[7] = b20 * a01 + b21 * a11 + b22 * a21;
// //    m.data[8] = b20 * a02 + b21 * a12 + b22 * a22;
//     m.data[0] = a00 * b00 + a01 * b10 + a02 * b20;
//     m.data[1] = a00 * b01 + a01 * b11 + a02 * b21;
//     m.data[2] = a00 * b02 + a01 * b12 + a02 * b22;
//     m.data[3] = a10 * b00 + a11 * b10 + a12 * b20;
//     m.data[4] = a10 * b01 + a11 * b11 + a12 * b21;
//     m.data[5] = a10 * b02 + a11 * b12 + a12 * b22;
//     m.data[6] = a20 * b00 + a21 * b10 + a22 * b20;
//     m.data[7] = a20 * b01 + a21 * b11 + a22 * b21;
//     m.data[8] = a20 * b02 + a21 * b12 + a22 * b22;
//     m
// }

// // Recibe una matriz 3x3 y un vector2 de P5 y devuelve su producto M x V en formato Vector2
// // El vector debe ser vector columna, devuelve vector2 raylib
// pub fn product_matrix_v3_columna(m: &Matrix3x3, v: &Vector3) -> Vector3 {
//     //println!("en product_matrix_v3 matriz_total = {:#?}", m);
//     let x = m.data[0] * v.x + m.data[1] * v.y + m.data[2];
//     let y = m.data[3] * v.x + m.data[4] * v.y + m.data[5];
//     //Vector2_fi::new(x, y)
//     Vector3::new(x, y, 0.0)
// }

// // Recibe una matriz 3x3 y un vector2 de P5 y devuelve su producto V x M en formato Vector2
// // El vector debe ser vector fila
// pub fn product_matrix_v3(m: &Matrix3x3, v: &Vector3) -> Vector3 {
//     //println!("en product_matrix_v3 matriz_total = {:#?}", m);
//     let x = m.data[0] * v.x + m.data[3] * v.y + m.data[6];
//     let y = m.data[1] * v.x + m.data[4] * v.y + m.data[7];
//     Vector3::new(x, y, 0.0)
// }

// Producto de matrices
impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, b: Matrix3x3) -> Self::Output {
        let mut m = identity3x3();

        let a00 = self.data[0];
        let a01 = self.data[1];
        let a02 = self.data[2];
        let a10 = self.data[3];
        let a11 = self.data[4];
        let a12 = self.data[5];
        let a20 = self.data[6];
        let a21 = self.data[7];
        let a22 = self.data[8];

        let b00 = b.data[0];
        let b01 = b.data[1];
        let b02 = b.data[2];
        let b10 = b.data[3];
        let b11 = b.data[4];
        let b12 = b.data[5];
        let b20 = b.data[6];
        let b21 = b.data[7];
        let b22 = b.data[8];

        m.data[0] = a00 * b00 + a01 * b10 + a02 * b20;
        m.data[1] = a00 * b01 + a01 * b11 + a02 * b21;
        m.data[2] = a00 * b02 + a01 * b12 + a02 * b22;
        m.data[3] = a10 * b00 + a11 * b10 + a12 * b20;
        m.data[4] = a10 * b01 + a11 * b11 + a12 * b21;
        m.data[5] = a10 * b02 + a11 * b12 + a12 * b22;
        m.data[6] = a20 * b00 + a21 * b10 + a22 * b20;
        m.data[7] = a20 * b01 + a21 * b11 + a22 * b21;
        m.data[8] = a20 * b02 + a21 * b12 + a22 * b22;
        m
    }
}

impl Mul<PVector3> for Matrix3x3 {
    type Output = PVector3;

    fn mul(self, v: PVector3) -> Self::Output {
        //println!("en product_matrix_v3 matriz_total = {:#?}", m);
        let x = self.data[0] * v.x + self.data[1] * v.y + self.data[2] * v.w;
        let y = self.data[3] * v.x + self.data[4] * v.y + self.data[5] * v.w;
        let w = self.data[6] * v.x + self.data[7] * v.y + self.data[8] * v.w;

        //Vector2_fi::new(x, y)
        pvector3(x, y, w)
    }
}

// Producto vectorial entre dos vectores
impl Mul for PVector3 {
    type Output = PVector3;

    fn mul(self, other: PVector3) -> PVector3 {
        PVector3 {
            x: self.y * other.w - self.w * other.y,
            y: self.w * other.x - self.x * other.w,
            w: self.x * other.y - self.y * other.x,
        }
    }
}

// Productor escalar entre dos vectores

pub fn mul_escalar(v0: PVector3, v1: PVector3) -> f32 {
    v0.x * v1.x + v0.y * v1.y + v0.w * v1.w
}

pub fn imprime_matriz_3x3(m: Matrix3x3) {
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━┓");

    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2} ┃", // ┃ \u{2503}
        m.data[0], m.data[1], m.data[2]
    );
    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2} ┃", // ┃ \u{2503}
        m.data[3], m.data[4], m.data[5]
    );
    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2} ┃", // ┃ \u{2503}
        m.data[6], m.data[7], m.data[8]
    );

    println!("┗━━━━━━━━━━━━━━━━━━━━━━━┛");
}

pub fn imprime_vector3(v: PVector3) {
    println!("┏━━━━━━━┓");
    println!("\u{2503}{0: >6.2} \u{2503}", v.x); // ┃      ┃
    println!("\u{2503}{0: >6.2} \u{2503}", v.y); // ┃      ┃
    println!("\u{2503}{0: >6.2} \u{2503}", v.w); // ┃      ┃
    println!("┗━━━━━━━┛");
}
