/*
Shape
    PShape
    createShape()
    loadShape()
2d Primitives
    arc()
    circle()
    ellipse()
    line()
    point()
    quad()
    rect()
    square()
    triangle()
Vertex
    beginContour()
    beginShape()
    bezierVertex()
    curveVertex()
    endContour()
    endShape()
    quadraticVertex()
    vertex()
Curves
    bezierDetail()
    bezierPoint()
    bezierTangent()
    bezier()
    curveDetail()
    curvePoint()
    curveTangent()
    curveTightness()
    curve()
3D Primitives
    box()
    sphereDetail()
    sphere()
Attributes
    ellipseMode()
    rectMode()
    strokeCap()
    strokeJoin()
    strokeWeight()
Loading & Display
    shapeMode()
    shape()
*/

use std::ops::Mul;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::parametros::{Parametros, RectMode};
use crate::matem::*;
use crate::transform::{identity3x3, Matrix3x3};

// Shape ************************************
struct PShape {}

pub fn create_shape() { unimplemented!(); }

pub fn load_shape() { unimplemented!(); }

// 2d Primitives *******************************

pub fn arc(canvas: &mut Canvas<Window>, param: &mut Parametros, cx: f32, cy: f32, r: f32, start_a: f32, end_a: f32) {
    let step = std::f32::consts::FRAC_PI_2 / 10.; // Tamaño del ángulo de cada línea recta

    let mut prev_x = cx + r * start_a.cos();
    let mut prev_y = cy - r * start_a.sin();

    // Creo un range para usar en f32
    let range = std::iter::successors(Some(start_a + step), move |&x| {
        if x + step < end_a {
            Some(x + step)
        } else {
            None
        }
    });

    for angle in range {
        let x = cx + r * angle.cos();
        let y = cy - r * angle.sin();

        // let st = Color::RGBA(
        //     param.stroke_color.r,
        //     param.stroke_color.g,
        //     param.stroke_color.b,
        //     param.stroke_color.a,
        // );
        let st_color = param.get_stroke_color_rgba_sdl2();
        let st = param.stroke_weight;
        canvas.thick_line(prev_x as i16, prev_y as i16, x as i16, y as i16, st, st_color);

        prev_x = x;
        prev_y = y;
    }
}

pub fn circle(canvas: &mut Canvas<Window>,
              param: &mut Parametros,
              x_vieja: f32,
              y_vieja: f32,
              radio: f32, ) {
    ellipse(canvas, param, x_vieja, y_vieja, radio, radio);
}

// Dibuja una elipse
pub fn ellipse(
    canvas: &mut Canvas<Window>,
    param: &mut Parametros,
    x_vieja: f32,
    y_vieja: f32,
    radio_ancho: f32,
    radio_alto: f32,
) {
    // Para la escala aplicamos matriz escala a los radios
    let p = param.matriz_total * pvector3(x_vieja, y_vieja, 1.0); // Es punto w = 1

    // Color de Stroke si se necesita
    // let st = Color::RGBA(
    //     param.stroke_color.r,
    //     param.stroke_color.g,
    //     param.stroke_color.b,
    //     param.stroke_color.a,
    // );

    let st_color = param.get_stroke_color_rgba_sdl2();

    // Color de relleno si se necesita
    /*let fi = Color::RGBA(
        param.fill_color.r,
        param.fill_color.g,
        param.fill_color.b,
        param.fill_color.a,
    );*/

    let fi_color = param.get_fill_color_rgba_sdl2();
    let error = 0.01f32; // Para comparaciones de f32
    if (param.stroke_weight as f32 - 1.0).abs() < error {
        // Valor por defecto
        if param.fill_bool {
            let _ = canvas.filled_ellipse(
                p.x as i16,
                p.y as i16,
                radio_ancho as i16,
                radio_alto as i16,
                fi_color,
            );
        }
        if param.stroke_bool {
            let _ = canvas.aa_ellipse(
                p.x as i16,
                p.y as i16,
                radio_ancho as i16,
                radio_alto as i16,
                st_color,
            );
        }
    } else {
        //# If the penwidth is larger than 1, things become a bit more complex.
        let inner_r1 = radio_ancho - param.stroke_weight as f32 * 0.5;
        let inner_r2 = radio_alto - param.stroke_weight as f32 * 0.5;

        let num_pasos = (2.0 * param.stroke_weight as f32) as usize;
        for n in 0..num_pasos {
            if param.fill_bool {
                let _ = canvas.filled_ellipse(
                    p.x as i16,
                    p.y as i16,
                    (inner_r1 + n as f32 * 0.5) as i16,
                    (inner_r2 + n as f32 * 0.5) as i16,
                    fi_color,
                );
            }
            if param.stroke_bool {
                let _ = canvas.aa_ellipse(
                    p.x as i16,
                    p.y as i16,
                    (inner_r1 + n as f32 * 0.5) as i16,
                    (inner_r2 + n as f32 * 0.5) as i16,
                    st_color,
                );
            }
        }
    }
}

// Dibuja una linea
pub fn line(
    canvas: &mut Canvas<Window>,
    param: &mut Parametros,
    x0_vieja: f32,
    y0_vieja: f32,
    x1_vieja: f32,
    y1_vieja: f32,
) {
    let p0 = param.matriz_total * pvector3(x0_vieja, y0_vieja, 1.0); // punto w = 1
    let p1 = param.matriz_total * pvector3(x1_vieja, y1_vieja, 1.0);

    // let st = Color::RGBA(
    //     param.stroke_color.r,
    //     param.stroke_color.g,
    //     param.stroke_color.b,
    //     param.stroke_color.a,
    // );

    let st_color = param.get_stroke_color_rgba_sdl2();

    let _ = canvas.thick_line(
        p0.x as i16,
        p0.y as i16,
        p1.x as i16,
        p1.y as i16,
        param.stroke_weight as u8,
        st_color,
    );
}

pub fn point(canvas: &mut Canvas<Window>, param: &Parametros, x: f32, y: f32) {
    // let c = Color::RGBA(
    //     param.stroke_color.r,
    //     param.stroke_color.g,
    //     param.stroke_color.b,
    //     param.stroke_color.a,
    // );

    let st_color = param.get_stroke_color_rgba_sdl2();
    let error = 0.01f32; // Para comparaciones de f32

    if (param.stroke_weight as f32 - 1.).abs() < error {
        let _ = canvas.pixel(x as i16, y as i16, st_color);
    } else {
        let _ = canvas.filled_circle(x as i16, y as i16, (param.stroke_weight as f32 / 2.0) as i16, st_color);
    }
}

// Dibuja un cuadrilátero
pub fn quad(
    canvas: &mut Canvas<Window>,
    param: &mut Parametros,
    x1_vieja: f32,
    y1_vieja: f32,
    x2_vieja: f32,
    y2_vieja: f32,
    x3_vieja: f32,
    y3_vieja: f32,
    x4_vieja: f32,
    y4_vieja: f32,
) {
    // let _p1 = param.matriz_total * pvector3(x1_vieja, y1_vieja, 1.0);
    // let _p2 = param.matriz_total * pvector3(x2_vieja, y2_vieja, 1.0);
    // let _p3 = param.matriz_total * pvector3(x3_vieja, y3_vieja, 1.0);
    // let _p4 = param.matriz_total * pvector3(x4_vieja, y4_vieja, 1.0);

    if param.stroke_bool {
        line(canvas, param, x1_vieja, y1_vieja, x2_vieja, y2_vieja);
        line(canvas, param, x2_vieja, y2_vieja, x3_vieja, y3_vieja);
        line(canvas, param, x3_vieja, y3_vieja, x4_vieja, y4_vieja);
        line(canvas, param, x4_vieja, y4_vieja, x1_vieja, y1_vieja);
    }
}

// Dibuja un rectángulo
pub fn rect(
    canvas: &mut Canvas<Window>,
    param: &mut Parametros,
    mut x_vieja: f32,
    mut y_vieja: f32,
    ancho: f32,
    alto: f32,
) {
    if param.rect_mode == RectMode::Center {
        x_vieja -= ancho / 2.;
        y_vieja -= alto / 2.;
    }

    // Para la escala aplicamos matriz escala al ancho y alto
    //println!("en shapes:ellipse antes ancho = {:#?} alto = {:#?}", ancho, alto);
    //let r = product_matrix_v3(&param.matriz_escala, &Vector2::new(ancho, alto));
    //println!("En shapes:rect   r={:#?}", r);

    let p = param.matriz_total * pvector3(x_vieja, y_vieja, 0.0);

    // let st = Color::RGBA(
    //     param.stroke_color.r,
    //     param.stroke_color.g,
    //     param.stroke_color.b,
    //     param.stroke_color.a,
    // );
    let st_color = param.get_stroke_color_rgba_sdl2();
    // let fi = Color::RGBA(
    //     param.fill_color.r,
    //     param.fill_color.g,
    //     param.fill_color.b,
    //     param.fill_color.a,
    // );

    let fi_color = param.get_fill_color_rgba_sdl2();

    let error = 0.01f32; // Para comparaciones de f32
    if (param.stroke_weight as f32 - 1.0).abs() < error {
        // Valor por defecto
        if param.fill_bool {
            canvas.set_draw_color(fi_color);
            let _ = canvas.fill_rect(sdl2::rect::Rect::new(
                p.x as i32,
                p.y as i32,
                ancho as u32,
                alto as u32,
            ));
        }

        if param.stroke_bool {
            let _ = canvas.rectangle(
                p.x as i16,
                p.y as i16,
                p.x as i16 + ancho as i16,
                p.y as i16 + alto as i16,
                st_color,
            );
        }
    } else {
        //# If the penwidth is larger than 1, things become a bit more complex.
        let inner_r1 = ancho - param.stroke_weight as f32 * 0.5;
        let inner_r2 = alto - param.stroke_weight as f32 * 0.5;

        let num_pasos = (2.0 * param.stroke_weight as f32) as usize;
        for n in 0..num_pasos {
            if param.fill_bool {
                let _ = canvas.filled_ellipse(
                    p.x as i16,
                    p.y as i16,
                    (inner_r1 + n as f32 * 0.5) as i16,
                    (inner_r2 + n as f32 * 0.5) as i16,
                    fi_color,
                );
            }
            if param.stroke_bool {
                let _ = canvas.aa_ellipse(
                    p.x as i16,
                    p.y as i16,
                    (inner_r1 + n as f32 * 0.5) as i16,
                    (inner_r2 + n as f32 * 0.5) as i16,
                    st_color,
                );
            }
        }
    }
    //canvas.present();  // No se si debe estar aqui
}

pub fn square(
    canvas: &mut Canvas<Window>,
    param: &mut Parametros,
    mut x_vieja: f32,
    mut y_vieja: f32,
    lado: f32,
) {
    rect(canvas, param, x_vieja, y_vieja, lado, lado)
}

pub fn triangle() { unimplemented!(); }

// Vertex ***********************************
pub fn begin_contour() { unimplemented!(); }

pub fn begin_shape() { unimplemented!(); }

pub fn bezier_vertex() { unimplemented!(); }

pub fn curve_vertex() { unimplemented!(); }

pub fn end_contour() { unimplemented!(); }

pub fn end_shape() { unimplemented!(); }

pub fn quadratic_vertex() { unimplemented!(); }

pub fn vertex() { unimplemented!(); }

// Curves ***************************************
pub fn bezier_detail() { unimplemented!(); }

pub fn bezier_point() { unimplemented!(); }

pub fn bezier_tangent() { unimplemented!(); }

pub fn bezier() { unimplemented!(); }

pub fn curve_detail() { unimplemented!(); }

pub fn curve_point() { unimplemented!(); }

pub fn curve_tangent() { unimplemented!(); }

pub fn curve_tightness() { unimplemented!(); }

pub fn curve() { unimplemented!(); }
// 3D Primitives ***********************************

// En Js es box lo renombro
pub fn boxy() { unimplemented!(); }

pub fn sphere_detail() { unimplemented!(); }

pub fn sphere() { unimplemented!(); }

// Attributes *****************************************
pub fn ellipse_mode() { unimplemented!(); }

pub fn rect_mode() { unimplemented!(); }

pub fn stroke_cap() { unimplemented!(); }

pub fn stroke_join() { unimplemented!(); }

// Indica la anchura de las lineas en pixels
pub fn stroke_weight(sw: u8, param: &mut Parametros) {
    param.stroke_weight = sw;
}

// Loading & Display ***********************************
pub fn shape_mode() { unimplemented!(); }

pub fn shape() { unimplemented!(); }

// *************************  Funciones creadas por mi ***********************

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

// 3d creadas por mi ************************************************************

// Sentido contrario a las agujas de reloj

pub fn draw_rectangulo_3d() { unimplemented!(); }

/*    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    va: Vector4,
    vb: Vector4,
    vc: Vector4,
    vd: Vector4,
) {
    // draw_triangulo_3d(d, param, va, vb, vc);
    // draw_triangulo_3d(d, param, vc, vd, va);
    // draw_linea_3d(d, param, va, vb);
    // draw_linea_3d(d, param, vb, vc);
    // draw_linea_3d(d, param, vc, vd);
    // draw_linea_3d(d, param, vd, va);

    // let camera = Camera3D::perspective(
    //     raylib::prelude::Vector3::new(param.camara.posx, param.camara.posy, param.camara.posz), // posición
    //     raylib::prelude::Vector3::new(param.camara.objx, param.camara.objy, param.camara.objz), // objetivo
    //     raylib::prelude::Vector3::new(param.camara.upx, param.camara.upy, param.camara.upz), // up
    //     param.camara.fovy,
    // );

    let camera = param.convierte_a_3d_raylib_persp();

    let p1 = param.matriz_total3d * Vector4::new(va.x, va.y, va.z, 1.0);
    let p2 = param.matriz_total3d * Vector4::new(vb.x, vb.y, vb.z, 1.0);
    let p3 = param.matriz_total3d * Vector4::new(vc.x, vc.y, vc.z, 1.0);
    let p4 = param.matriz_total3d * Vector4::new(vd.x, vd.y, vd.z, 1.0);
    // Modo 3D

    if param.fill_bool {
        let mut d3 = d.begin_mode3D(camera);
        d3.draw_triangle3D(
            // lado 1
            p1.to_v3_raylib(),
            p2.to_v3_raylib(),
            p3.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
        d3.draw_triangle3D(
            // lado 2
            p3.to_v3_raylib(),
            p2.to_v3_raylib(),
            p1.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
        // Segundo triangulo ------------------------------------------------
        d3.draw_triangle3D(
            // lado 1
            p3.to_v3_raylib(),
            p4.to_v3_raylib(),
            p1.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
        d3.draw_triangle3D(
            // lado 2
            p1.to_v3_raylib(),
            p4.to_v3_raylib(),
            p3.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
    }

    if param.stroke_bool {
        draw_linea_3d(d, param, va, vb);
        draw_linea_3d(d, param, vb, vc);
        draw_linea_3d(d, param, vc, vd);
        draw_linea_3d(d, param, vd, va);
    }
}*/

// Esta funcion es un cubo con puntos creado desde triangulos para que le afecten a todos los puntos las matrices
// la posicion es el centro del cubo
pub fn cubo3d() { unimplemented!(); }
/*pub fn cubo3d(d: &mut RaylibDrawHandle, param: &mut Parametros, lado: f32) {
    let slado = lado / 2.0;
    let puntos: Vec<Vector4> = vec![
        Vector4::new(-slado, -slado, slado, 1.0),  // 0
        Vector4::new(slado, -slado, slado, 1.0),   // 1
        Vector4::new(slado, slado, slado, 1.0),    // 2
        Vector4::new(-slado, slado, slado, 1.0),   // 3
        Vector4::new(-slado, -slado, -slado, 1.0), // 4
        Vector4::new(slado, -slado, -slado, 1.0),  // 5
        Vector4::new(slado, slado, -slado, 1.0),   // 6
        Vector4::new(-slado, slado, -slado, 1.0),  // 7
    ];

    // la matriz total3d, se aplica al renderizar los triangulos

    draw_rectangulo_3d(d, param, puntos[0], puntos[1], puntos[2], puntos[3]);
    draw_rectangulo_3d(d, param, puntos[1], puntos[5], puntos[6], puntos[2]);
    draw_rectangulo_3d(d, param, puntos[5], puntos[4], puntos[7], puntos[6]);
    draw_rectangulo_3d(d, param, puntos[0], puntos[3], puntos[7], puntos[4]);
    draw_rectangulo_3d(d, param, puntos[2], puntos[6], puntos[7], puntos[3]);
    draw_rectangulo_3d(d, param, puntos[0], puntos[4], puntos[5], puntos[1]);
}*/

