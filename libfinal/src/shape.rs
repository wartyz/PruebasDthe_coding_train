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
use crate::parametros::{ModosBeginShape, Parametros, RectMode};
use crate::matem::*;
use crate::parametros::ModosBeginShape::{Close, Lines};
use crate::transform::{identity3x3, Matrix3x3};

// Shape ************************************
struct _PShape {}

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
    let st_color = param.get_stroke_color_rgba_sdl2();

    // Color de relleno si se necesita
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

    let st_color = param.get_stroke_color_rgba_sdl2();

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
    x_vieja: f32,
    y_vieja: f32,
    lado: f32,
) {
    rect(canvas, param, x_vieja, y_vieja, lado, lado)
}

pub fn triangle() { unimplemented!(); }

// Vertex ***********************************
pub fn begin_contour() { unimplemented!(); }

pub fn begin_shape(_shape: ModosBeginShape) {}

pub fn bezier_vertex() { unimplemented!(); }

pub fn curve_vertex() { unimplemented!(); }

pub fn end_contour() { unimplemented!(); }

pub fn end_shape(canvas: &mut Canvas<Window>, param: &mut Parametros, modo_close: ModosBeginShape) {
    let mut vx = Vec::new();
    let mut vy = Vec::new();

    for v in &param.vertex {
        let p = param.matriz_total * pvector3(v.x, v.y, 1.); // ojo 1. si no da error

        vx.push(p.x as i16);
        vy.push(p.y as i16);
    }

    // Poner el inicio de nuevo
    // let p = param.matriz_total * pvector3(param.vertex[0].x, param.vertex[0].y, 1.);
    // vx.push(p.x as i16);
    // vy.push(p.y as i16);

    let st = param.get_stroke_color_rgba_sdl2();
    let fi_color = param.get_fill_color_rgba_sdl2();

    if modo_close == Lines {
        let _ = canvas.aa_polygon(&vx, &vy, st);
    }
    if modo_close == Close {
        let _ = canvas.filled_polygon(&vx, &vy, fi_color);
    }

    param.vertex.clear();
}

pub fn quadratic_vertex() { unimplemented!(); }

pub fn vertex(x_vieja: f32, y_vieja: f32, param: &mut Parametros) {
    param.vertex.push(pvector2(x_vieja, y_vieja));
}

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

pub fn rect_mode(mode: RectMode, param: &mut Parametros) {
    param.rect_mode = mode;
}

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

// pub fn imprime_vector4(v: PVector4) {
//     println!("┏━━━━━━━━┓");
//     println!("\u{2503} {0: >6.2} \u{2503}", v.x); // ┃      ┃
//     println!("\u{2503} {0: >6.2} \u{2503}", v.y); // ┃      ┃
//     println!("\u{2503} {0: >6.2} \u{2503}", v.z); // ┃      ┃
//     println!("\u{2503} {0: >6.2} \u{2503}", v.w); // ┃      ┃
//     println!("┗━━━━━━━━┛");
// }

// 3d creadas por mi ************************************************************

// Sentido contrario a las agujas de reloj

pub fn draw_rectangulo_3d(_param: &mut Parametros,
                          _va: PVector4,
                          _vb: PVector4,
                          _vc: PVector4,
                          _vd: PVector4, ) {
    //draw_triangulo_3d(param, va, vb, vc);
}

// Esta funcion es un cubo con puntos creado desde triangulos para que le afecten a todos los puntos las matrices
// la posicion es el centro del cubo

pub fn cubo3d(_param: &mut Parametros, lado: f32) {
    let slado = lado / 2.0;
    let _puntos: Vec<PVector4> = vec![
        pvector4(-slado, -slado, slado, 1.0),  // 0
        pvector4(slado, -slado, slado, 1.0),   // 1
        pvector4(slado, slado, slado, 1.0),    // 2
        pvector4(-slado, slado, slado, 1.0),   // 3
        pvector4(-slado, -slado, -slado, 1.0), // 4
        pvector4(slado, -slado, -slado, 1.0),  // 5
        pvector4(slado, slado, -slado, 1.0),   // 6
        pvector4(-slado, slado, -slado, 1.0),  // 7
    ];

    // la matriz total3d, se aplica al renderizar los triangulos

    //draw_rectangulo_3d(param, puntos[0], puntos[1], puntos[2], puntos[3]);
    // draw_rectangulo_3d(d, param, puntos[1], puntos[5], puntos[6], puntos[2]);
    // draw_rectangulo_3d(d, param, puntos[5], puntos[4], puntos[7], puntos[6]);
    // draw_rectangulo_3d(d, param, puntos[0], puntos[3], puntos[7], puntos[4]);
    // draw_rectangulo_3d(d, param, puntos[2], puntos[6], puntos[7], puntos[3]);
    // draw_rectangulo_3d(d, param, puntos[0], puntos[4], puntos[5], puntos[1]);
}

