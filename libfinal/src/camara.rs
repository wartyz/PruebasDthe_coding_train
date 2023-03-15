use crate::matem::{pvector3, PVector3, sub_s};
use crate::transform3d::{identity4x4, Matrix4x4};
use crate::transform::mul_escalar;

use glam::{Mat4, Vec3, Affine2, Affine3A};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::gfx::primitives::DrawRenderer;
use crate::parametros::Parametros;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            eye: Vec3::new(0.0, 0.0, -5.0),
            center: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        }
    }
    pub fn new3(eye: Vec3, center: Vec3, up: Vec3) -> Camera {
        Camera { eye, center, up }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye, self.center, self.up)
    }

    pub fn projection_matrix(w_ancho: f32, w_alto: f32) -> Mat4 {
        Mat4::perspective_rh(
            45.0f32.to_radians(),
            w_ancho / w_alto,
            0.1,
            100.0,
        )
    }

    pub fn dibujar_cubo(canvas: &mut Canvas<Window>, _param: &mut Parametros) {

        // Limpiar la pantalla
        canvas.set_draw_color(Color::RGB(100, 100, 0));
        canvas.clear();

        // Definir los vertices del cubo
        let v0 = Vec3::new(-100.0, -100.0, -100.0);
        let v1 = Vec3::new(100.0, -100.0, -100.0);
        let v2 = Vec3::new(100.0, 100.0, -100.0);
        let v3 = Vec3::new(-100.0, 100.0, -100.0);
        let v4 = Vec3::new(-100.0, -100.0, 100.0);
        let v5 = Vec3::new(100.0, -100.0, 100.0);
        let v6 = Vec3::new(100.0, 100.0, 100.0);
        let v7 = Vec3::new(-100.0, 100.0, 100.0);

        // Definir las caras del cubo como índices a los vertices
        let faces = [
            // Cara frontal
            (v0, v1),
            (v1, v2),
            (v2, v3),
            (v3, v0),
            // Cara trasera
            (v4, v5),
            (v5, v6),
            (v6, v7),
            (v7, v4),
            // Caras laterales
            (v0, v4),
            (v1, v5),
            (v2, v6),
            (v3, v7),
        ];

        // Obtener la matriz de transformación del cubo (puede ser rotada, escalada, etc.)
        let transform =
            Affine3A::from_rotation_x(0.5)
                * Affine3A::from_rotation_y(0.5)
                * Affine3A::from_translation(Vec3::new(0.0, 0.0, -5.0));

        // Dibujar cada línea del cubo
        for &(v0, v1) in &faces {
            let p0 = transform.transform_point3(v0);
            let p1 = transform.transform_point3(v1);

            let st_color = Color::RGB(255, 0, 0);

            let _ = canvas.thick_line(
                (p0.x + 400.) as i16,
                (p0.y + 300.) as i16,
                (p1.x + 400.) as i16,
                (p1.y + 300.) as i16,
                4,
                st_color,
            );
        }
    }

    // }
    //
    // #[rustfmt::skip]
    // pub fn view_matrix(&self, eye: PVector3, target: PVector3, up: PVector3) -> Matrix4x4 {
    //     let mut forward = sub_s(&target, &eye);
    //     forward.normalize();
    //     // cross es producto vectorial
    //     //let side = (cross(forward, up)).normalize();
    //     let side = forward * up;
    //
    //     //let new_up = (side * forward);
    //     let new_up = side * forward;
    //
    //     let view = Matrix4x4 {
    //         data: [side.x, side.y, side.w, -mul_escalar(side, eye),
    //             new_up.x, new_up.y, new_up.w, -mul_escalar(new_up, eye),
    //             -forward.x, -forward.y, -forward.w, mul_escalar(forward, eye),
    //             0.0, 0.0, 0.0, 1.0]
    //     };
    //
    //     view
}



