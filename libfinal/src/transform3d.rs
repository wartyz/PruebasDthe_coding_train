use std::ops::Mul;
use crate::matem::{PVector4, pvector4};
use crate::parametros::Parametros;
use crate::transform::{identity3x3, Matrix3x3};

// USO VECTOR COLUMNA
// Ojo para el cálculo la matriz es 0  1  2  3  y el vector  x
//                                  4  5  6  7               y
//                                  8  9 10 11               z
//                                 12 13 14 15               1

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4 {
    pub data: [f32; 16],
}

impl Matrix4x4 {
    #[rustfmt::skip]
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m04: f32, m05: f32, m06: f32, m07: f32,
        m08: f32, m09: f32, m10: f32, m11: f32,
        m12: f32, m13: f32, m14: f32, m15: f32,
    ) -> Self {
        Matrix4x4 {
            data: [m00, m01, m02, m03,
                m04, m05, m06, m07,
                m08, m09, m10, m11,
                m12, m13, m14, m15, ]
        }
    }

    // Translación respecto a los ejes globales con self
    #[rustfmt::skip]
    pub fn translate3d(&mut self, x: f32, y: f32, z: f32) {
        let m = Matrix4x4 {
            data: [1.0, 0.0, 0.0, x,
                0.0, 1.0, 0.0, y,
                0.0, 0.0, 1.0, z,
                0.0, 0.0, 0.0, 1.0]
        };

        *self = *self * m;
    }

    // Escala igual en todos los ejes
    #[rustfmt::skip]
    pub fn scale1_3d(&mut self, v: f32) {
        let m = Matrix4x4 {
            data: [v, 0.0, 0.0, 0.0,
                0.0, v, 0.0, 0.0,
                0.0, 0.0, v, 0.0,
                0.0, 0.0, 0.0, 1.0, ]
        };

        *self = *self * m;
    }

    #[rustfmt::skip]
    pub fn scale3_3d(&mut self, x: f32, y: f32, z: f32) {
        let m = Matrix4x4 {
            data: [x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0, ]
        };

        *self = *self * m;
    }

    #[rustfmt::skip]
    pub fn rotate_y3d(&mut self, angulo: f32) {
        let m = Matrix4x4 {
            data: [angulo.cos(), 0.0, angulo.sin(), 0.0,
                0.0, 1.0, 0.0, 0.0,
                -angulo.sin(), 0.0, angulo.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0]
        };

        *self = *self * m;
    }

    #[rustfmt::skip]
    pub fn rotate_z3d(&mut self, angulo: f32) {
        let m = Matrix4x4 {
            data: [angulo.cos(), -angulo.sin(), 0.0, 0.0,
                angulo.sin(), angulo.cos(), 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0]
        };

        *self = *self * m;
    }

    #[rustfmt::skip]
    pub fn rotate_x3d(&mut self, angulo: f32) {
        let m = Matrix4x4 {
            data: [1.0, 0.0, 0.0, 0.0,
                0.0, angulo.cos(), -angulo.sin(), 0.0,
                0.0, angulo.sin(), angulo.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0]
        };

        *self = *self * m;
    }

    pub fn submatrix3d(&self, row: usize, column: usize) -> Matrix3x3 {
        let mut matrix3x3 = identity3x3();
        let mut source_row: usize = 0;
        let mut source_column: usize = 0;
        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < 3 {
            if source_row == row {
                // Skip row to be removed
                source_row += 1;
            }
            while target_column < 3 {
                if source_column == column {
                    // Skip column to be removed
                    source_column += 1;
                }
                matrix3x3.data[target_column + target_row * 3] =
                    self.data[source_column + source_row * 4];

                source_column += 1;
                target_column += 1;
            }
            source_row += 1;
            source_column = 0;
            target_row += 1;
            target_column = 0;
        }
        matrix3x3
    }

    pub fn minor3d(&self, row: usize, column: usize) -> f32 {
        self.submatrix3d(row, column).determinant()
    }

    pub fn cofactor3d(&self, row: usize, column: usize) -> f32 {
        let minor = self.minor3d(row, column);
        if (row + column) % 2 == 0 {
            // Even value
            minor
        } else {
            -minor
        }
    }
    pub fn determinant3d(&self) -> f32 {
        let mut determinant = 0.0;
        for column in 0..4 {
            determinant += self.cofactor3d(0, column) * self.data[column];
        }

        determinant
    }

    // Cuidado, no estoy muy seguro-----------------------
    pub fn is_invertible3d(&self) -> bool {
        self.determinant3d() != 0.0
    }

    pub fn inverse3d(&self) -> Matrix4x4 {
        if !self.is_invertible3d() {
            panic!("La matriz no es invertible, pero se ha llamado a inverse()");
        }

        let mut matrix = identity4x4();
        let determinant = self.determinant3d();

        for row in 0..4 {
            for column in 0..4 {
                let cofactor = self.cofactor3d(row, column);
                // transposed storage
                matrix.data[row + column * 4] = cofactor / determinant;
            }
        }

        matrix
    }

    pub fn transpuesta3d(&self) -> Matrix4x4 {
        let mut matrix = identity4x4();
        for row in 0..4 {
            for column in 0..4 {
                matrix.data[row + column * 4] = self.data[column + row * 4];
            }
        }
        matrix
    }
}

#[rustfmt::skip]
pub fn identity4x4() -> Matrix4x4 {
    Matrix4x4::new(1.0, 0.0, 0.0, 0.0,
                   0.0, 1.0, 0.0, 0.0,
                   0.0, 0.0, 1.0, 0.0,
                   0.0, 0.0, 0.0, 1.0, )
}

// Escala igual en todos los ejes
#[rustfmt::skip]
pub fn scale1_3d(v: f32, param: &mut Parametros) {
    let m = Matrix4x4 {
        data: [v, 0.0, 0.0, 0.0,
            0.0, v, 0.0, 0.0,
            0.0, 0.0, v, 0.0,
            0.0, 0.0, 0.0, 1.0, ]
    };

    param.matriz_total3d = param.matriz_total3d * m
}

// Translación respecto a los ejes globales
#[rustfmt::skip]
pub fn translate3d(x: f32, y: f32, z: f32, param: &mut Parametros) {
    let m = Matrix4x4 {
        data: [1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0]
    };

    param.matriz_total3d = param.matriz_total3d * m;
}

// Experimento Translación respecto a los ejes globales
#[rustfmt::skip]
pub fn translate3d_inversa(x: f32, y: f32, z: f32, param: &mut Parametros) {
    let m = Matrix4x4 {
        data: [1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0]
    };

    param.matriz_total3d = param.matriz_total3d * m;
}

// carga la matriz de rotacion del eje x con una coordenada
#[rustfmt::skip]
pub fn rotate_x3d(angulo: f32, param: &mut Parametros) {
    let m = Matrix4x4 {
        data: [1.0, 0.0, 0.0, 0.0,
            0.0, angulo.cos(), -angulo.sin(), 0.0,
            0.0, angulo.sin(), angulo.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0]
    };

    param.matriz_total3d = param.matriz_total3d * m;
}

// carga la matriz de rotacion del eje y con una coordenada
#[rustfmt::skip]
pub fn rotate_y3d(angulo: f32, param: &mut Parametros) {
    let m = Matrix4x4 {
        data: [angulo.cos(), 0.0, angulo.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -angulo.sin(), 0.0, angulo.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0]
    };

    param.matriz_total3d = param.matriz_total3d * m;
}

// Recibe un ángulo en radianes y carga la matriz de rotacion del eje z
#[rustfmt::skip]
pub fn rotate_z3d(angulo: f32, param: &mut Parametros) {
    let m = Matrix4x4 {
        data: [angulo.cos(), -angulo.sin(), 0.0, 0.0,
            angulo.sin(), angulo.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0]
    };

    param.matriz_total3d = param.matriz_total3d * m;
}

// Producto de matrices
impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, b: Matrix4x4) -> Self::Output {
        let mut m = identity4x4();

        let a00 = self.data[0];
        let a01 = self.data[1];
        let a02 = self.data[2];
        let a03 = self.data[3];

        let a10 = self.data[4];
        let a11 = self.data[5];
        let a12 = self.data[6];
        let a13 = self.data[7];

        let a20 = self.data[8];
        let a21 = self.data[9];
        let a22 = self.data[10];
        let a23 = self.data[11];

        let a30 = self.data[12];
        let a31 = self.data[13];
        let a32 = self.data[14];
        let a33 = self.data[15];

        let b00 = b.data[0];
        let b01 = b.data[1];
        let b02 = b.data[2];
        let b03 = b.data[3];

        let b10 = b.data[4];
        let b11 = b.data[5];
        let b12 = b.data[6];
        let b13 = b.data[7];

        let b20 = b.data[8];
        let b21 = b.data[9];
        let b22 = b.data[10];
        let b23 = b.data[11];

        let b30 = b.data[12];
        let b31 = b.data[13];
        let b32 = b.data[14];
        let b33 = b.data[15];

        m.data[0] = a00 * b00 + a01 * b10 + a02 * b20 + a03 * b30;
        m.data[1] = a00 * b01 + a01 * b11 + a02 * b21 + a03 * b31;
        m.data[2] = a00 * b02 + a01 * b12 + a02 * b22 + a03 * b32;
        m.data[3] = a00 * b03 + a01 * b13 + a02 * b23 + a03 * b33;

        m.data[4] = a10 * b00 + a11 * b10 + a12 * b20 + a13 * b30;
        m.data[5] = a10 * b01 + a11 * b11 + a12 * b21 + a13 * b31;
        m.data[6] = a10 * b02 + a11 * b12 + a12 * b22 + a13 * b32;
        m.data[7] = a10 * b03 + a11 * b13 + a12 * b23 + a13 * b33;

        m.data[8] = a20 * b00 + a21 * b10 + a22 * b20 + a23 * b30;
        m.data[9] = a20 * b01 + a21 * b11 + a22 * b21 + a23 * b31;
        m.data[10] = a20 * b02 + a21 * b12 + a22 * b22 + a23 * b32;
        m.data[11] = a20 * b03 + a21 * b13 + a22 * b23 + a23 * b33;

        m.data[12] = a30 * b00 + a31 * b10 + a32 * b20 + a33 * b30;
        m.data[13] = a30 * b01 + a31 * b11 + a32 * b21 + a33 * b31;
        m.data[14] = a30 * b02 + a31 * b12 + a32 * b22 + a33 * b32;
        m.data[15] = a30 * b03 + a31 * b13 + a32 * b23 + a33 * b33;

        m
    }
}

impl Mul<PVector4> for Matrix4x4 {
    type Output = PVector4;

    #[rustfmt::skip]
    fn mul(self, v: PVector4) -> Self::Output {
        let x = self.data[0] * v.x + self.data[1] * v.y + self.data[2] * v.z + self.data[3] * v.w;
        let y = self.data[4] * v.x + self.data[5] * v.y + self.data[6] * v.z + self.data[7] * v.w;
        let z = self.data[8] * v.x + self.data[9] * v.y + self.data[10] * v.z + self.data[11] * v.w;
        let w = self.data[12] * v.x + self.data[13] * v.y + self.data[14] * v.z + self.data[15] * v.w;

        pvector4(x, y, z, w)
    }
}

// Pone la matriz de transformación en la pila de matriz.
pub fn push_matrix3d(param: &mut Parametros) {
    param.matriz_stack3d.push(param.matriz_total3d);
    // prueba
    //param.matriz_total3d = identity4x4(); // -------------prueba ----------------------------------
}

pub fn pop_matrix3d(param: &mut Parametros) {
    param.matriz_total3d = param.matriz_stack3d.pop().unwrap();
}

pub fn apply_matrix3d(param: &mut Parametros, matrix: Matrix4x4) {
    param.matriz_total3d = matrix * param.matriz_total3d;
}

pub fn imprime_matriz_4x4(m: Matrix4x4) {
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");

    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2}  {3: >6.2} ┃", // ┃ \u{2503}
        m.data[0], m.data[1], m.data[2], m.data[3]
    );
    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2}  {3: >6.2} ┃", // ┃ \u{2503}
        m.data[4], m.data[5], m.data[6], m.data[7]
    );
    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2}  {3: >6.2} ┃", // ┃ \u{2503}
        m.data[8], m.data[9], m.data[10], m.data[11]
    );

    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2}  {3: >6.2} ┃", // ┃ \u{2503}
        m.data[12], m.data[13], m.data[14], m.data[15]
    );
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
}

pub fn imprime_vector4(v: PVector4) {
    println!("┏━━━━━━━━┓");
    println!("\u{2503} {0: >6.2} \u{2503}", v.x); // ┃      ┃
    println!("\u{2503} {0: >6.2} \u{2503}", v.y); // ┃      ┃
    println!("\u{2503} {0: >6.2} \u{2503}", v.z); // ┃      ┃
    println!("\u{2503} {0: >6.2} \u{2503}", v.w); // ┃      ┃
    println!("┗━━━━━━━━┛");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn pba() {
        assert_eq!(5, 5);
    }

    #[test]
    #[rustfmt::skip]
    fn construyendo_e_inspeccionando_una_matriz_4x4() {
        let m = Matrix4x4::new(1.0, 2.0, 3.0, 7.3,
                               5.5, 6.5, 7.5, 8.2,
                               9.0, 10.0, 11.0, 5.9,
                               12.0, 7.55, 7.3, 9.2);

        assert_eq!(m.data[0], 1.0);
        assert_eq!(m.data[1], 2.0);
        assert_eq!(m.data[2], 3.0);
        assert_eq!(m.data[3], 7.3);
        assert_eq!(m.data[4], 5.5);
        assert_eq!(m.data[5], 6.5);
        assert_eq!(m.data[6], 7.5);
        assert_eq!(m.data[7], 8.2);
        assert_eq!(m.data[8], 9.0);
        assert_eq!(m.data[9], 10.0);
        assert_eq!(m.data[10], 11.0);
        assert_eq!(m.data[11], 5.9);
        assert_eq!(m.data[12], 12.0);
        assert_eq!(m.data[13], 7.55);
        assert_eq!(m.data[14], 7.3);
        assert_eq!(m.data[15], 9.2);
    }

    #[test]
    #[rustfmt::skip]
    fn imprimiendo_una_matriz_4x4() {
        let m = Matrix4x4::new(1.0, 2.0, 3.0, 8.6,
                               5.5, 6.5, 7.5, 3.9,
                               9.0, 10.0, 11.0, 5.1,
                               12.9, 45.7, 29.5, 123.8);
        imprime_matriz_4x4(m);
    }

    #[test]
    fn imprimiendo_un_vector4() {
        let v = Vector4::new(5.97432, 6.484, 9.55, 6.33);
        imprime_vector4(v);
    }

    #[test]
    #[rustfmt::skip]
    fn producto_matrix4_por_matrix4_igual_a_matrix4_con_asterisco() {
        let m0 = Matrix4x4::new(1.0, 2.0, 3.0, 5.6,
                                5.5, 6.5, 7.5, 8.2,
                                9.0, 10.0, 11.0, 6.5,
                                2.9, 5.2, 9.4, 7.2);

        let m1 = Matrix4x4::new(0.67, 5.0, 7.0, 7.9,
                                5.88, 6.5, 7.5, 9.2,
                                4.0, 6.0, 6.7, 2.0,
                                9.4, 8.3, 7.3, 7.1);

        println!("    m0     ");
        imprime_matriz_4x4(m0);

        println!("    m1     ");
        imprime_matriz_4x4(m1);

        println!("   m0 * m1   =");
        imprime_matriz_4x4(m0 * m1);

        println!("   m1 * m0   =");
        imprime_matriz_4x4(m1 * m0);
    }

    //      m0 * m1   =
    //     ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
    //     ┃ 77.07   82.48   82.98   72.06 ┃
    //     ┃148.98  182.81  197.36  176.47 ┃
    //     ┃169.93  229.95  259.15  231.25 ┃
    //     ┃137.80  164.46  174.84  140.67 ┃
    //     ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    //      m1 * m0   =
    //     ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
    //     ┃114.08  144.92  190.77  147.13 ┃
    //     ┃135.81  176.85  235.37  201.22 ┃
    //     ┃103.10  124.40  149.50  129.55 ┃
    //     ┃141.34  182.67  237.49  219.27 ┃
    //     ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

    #[test]
    #[rustfmt::skip]
    fn producto_matrix4_por_vector4_igual_a_vector4_con_asterisco() {
        let m0 = Matrix4x4::new(1.0, 2.0, 3.0, 8.5,
                                5.5, 6.5, 7.5, 7.2,
                                9.0, 10.0, 11.0, 2.9,
                                12.8, 4.8, 9.5, 9.3);
        let v1 = Vector4::new(4.0, 7.0, 3.8, 9.4);

        println!("   m0 * v1   =");
        imprime_vector4(m0 * v1);

        //   m0 * v1   =
        //    ┏━━━━━━━━┓
        //    ┃ 109.30 ┃
        //    ┃ 163.68 ┃
        //    ┃ 175.06 ┃
        //    ┃ 208.32 ┃
        //    ┗━━━━━━━━┛
    }

    #[test]
    #[rustfmt::skip]
    fn producto_matrix4_identity_por_vector4_igual_a_vector4_con_asterisco() {
        let m0 = identity4x4();
        let v1 = Vector4::new(4.0, 7.0, 3.8, 9.4);

        println!("   m0 * v1   =");
        imprime_vector4(m0 * v1);
    }

    #[test]
    #[rustfmt::skip]
    fn imprime_transpuesta() {
        let m0 = Matrix4x4::new(1.0, 2.0, 3.0, 8.5,
                                5.5, 6.5, 7.5, 7.2,
                                9.0, 10.0, 11.0, 2.9,
                                12.8, 4.8, 9.5, 9.3);
        let trans = m0.transpuesta3d();
        println!("   m0   =");
        imprime_matriz_4x4(m0);
        println!("   transpuesta de m0   =");
        imprime_matriz_4x4(trans);
    }

    #[test]
    #[rustfmt::skip]
    fn pruebas_push_matrix() {
        let mut par = Parametros::new(500.0, 500.0);

        par.matriz_total3d = Matrix4x4::new(1.0, 2.0, 3.0, 8.5,
                                            5.5, 6.5, 7.5, 7.2,
                                            9.0, 10.0, 11.0, 2.9,
                                            12.8, 4.8, 9.5, 9.3);

        let b = Matrix4x4::new(0.67, 5.0, 7.0, 7.9,
                               5.88, 6.5, 7.5, 9.2,
                               4.0, 6.0, 6.7, 2.0,
                               9.4, 8.3, 7.3, 7.1);

        println!("matriz_total3d antes de push =");
        imprime_matriz_4x4(par.matriz_total3d);

        println!("hacemos push");
        push_matrix3d(&mut par);

        println!("matriz_total3d despues de push =");
        imprime_matriz_4x4(par.matriz_total3d);

        println!("matriz_total3d = matriz_total3d * b=");
        par.matriz_total3d = par.matriz_total3d * b;
        imprime_matriz_4x4(par.matriz_total3d);

        println!("hacemos pop");
        pop_matrix3d(&mut par);
        println!("despues de hacer pop =");
        imprime_matriz_4x4(par.matriz_total3d);
    }

    #[test]
    fn pruebas_orden_traslate_rotate() {
        let p_inicial1 = Vector4::new(100.0, 100.0, 0.0, 1.0);

        let mut t1 = identity4x4();
        t1.translate3d(100.0, 0.0, 0.0);
        t1.rotate_x3d(PI / 4.0);
        let p_tr: Vector4 = t1 * p_inicial1;

        let p_inicial2 = Vector4::new(100.0, 100.0, 0.0, 1.0);
        let mut t2 = identity4x4();
        t2.rotate_x3d(PI / 4.0);
        t1.translate3d(100.0, 0.0, 0.0);
        let p_rt: Vector4 = t2 * p_inicial2;

        println!("p_tr =");
        imprime_vector4(p_tr);

        println!("p_rt =");
        imprime_vector4(p_rt);
    }
}
