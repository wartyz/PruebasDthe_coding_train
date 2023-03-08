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

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::parametros::Parametros;
use crate::matem::*;

// Shape ************************************
struct PShape {}

pub fn create_shape() { unimplemented!(); }

pub fn load_shape() { unimplemented!(); }

// 2d Primitives *******************************
pub fn arc() { unimplemented!(); }

pub fn circle() { unimplemented!(); }

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
    //println!("en shapes:ellipse antes ancho = {:#?} alto = {:#?}", ancho, alto);
    //let r = product_matrix_v3(&param.matriz_escala, &Vector2::new(ancho / 2.0, alto / 2.0));
    //println!("en shapes:ellipse despues  r = {:#?}", r);

    //transform(param); // Carga la matriz resultado de todas las matrices en param.matriz_total

    //println!("en shapes:ellipse matriz_total = {:#?}", param.matriz_total);

    let p = param.matriz_total * pvector3(x_vieja, y_vieja, 1.0); // Es punto w = 1

    let st = sdl2::pixels::Color::RGBA(
        param.stroke_color.r,
        param.stroke_color.g,
        param.stroke_color.b,
        param.stroke_color.a,
    );

    let fi = sdl2::pixels::Color::RGBA(
        param.fill_color.r,
        param.fill_color.g,
        param.fill_color.b,
        param.fill_color.a,
    );

    let error = 0.01f32; // Para comparaciones de f32
    if (param.stroke_weight - 1.0).abs() < error {
        // Valor por defecto
        if param.fill_bool {
            let _ = canvas.filled_ellipse(
                p.x as i16,
                p.y as i16,
                radio_ancho as i16,
                radio_alto as i16,
                fi,
            );
        }
        if param.stroke_bool {
            let _ = canvas.aa_ellipse(
                p.x as i16,
                p.y as i16,
                radio_ancho as i16,
                radio_alto as i16,
                st,
            );
        }
    } else {
        //# If the penwidth is larger than 1, things become a bit more complex.
        let inner_r1 = radio_ancho - param.stroke_weight * 0.5;
        let inner_r2 = radio_alto - param.stroke_weight * 0.5;

        let num_pasos = (2.0 * param.stroke_weight) as usize;
        for n in 0..num_pasos {
            if param.fill_bool {
                let _ = canvas.filled_ellipse(
                    p.x as i16,
                    p.y as i16,
                    (inner_r1 + n as f32 * 0.5) as i16,
                    (inner_r2 + n as f32 * 0.5) as i16,
                    fi,
                );
            }
            if param.stroke_bool {
                let _ = canvas.aa_ellipse(
                    p.x as i16,
                    p.y as i16,
                    (inner_r1 + n as f32 * 0.5) as i16,
                    (inner_r2 + n as f32 * 0.5) as i16,
                    st,
                );
            }
        }
    }
}

/// Dibuja una linea
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

    let st = sdl2::pixels::Color::RGBA(
        param.stroke_color.r,
        param.stroke_color.g,
        param.stroke_color.b,
        param.stroke_color.a,
    );

    let _ = canvas.thick_line(
        p0.x as i16,
        p0.y as i16,
        p1.x as i16,
        p1.y as i16,
        param.stroke_weight as u8,
        st,
    );
}

pub fn point() { unimplemented!(); }

pub fn quad() { unimplemented!(); }

pub fn rect() { unimplemented!(); }

pub fn square() { unimplemented!(); }

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
pub fn boxa() { unimplemented!(); }

pub fn sphere_detail() { unimplemented!(); }

pub fn sphere() { unimplemented!(); }

// Attributes *****************************************
pub fn ellipse_mode() { unimplemented!(); }

pub fn rect_mode() { unimplemented!(); }

pub fn stroke_cap() { unimplemented!(); }

pub fn stroke_join() { unimplemented!(); }

pub fn stroke_weight() { unimplemented!(); }

// Loading & Display ***********************************
pub fn shape_mode() { unimplemented!(); }

pub fn shape() { unimplemented!(); }