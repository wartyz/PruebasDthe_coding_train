use std::thread;
use crate::matem::{pvector3, PVector3, sub_s};
use crate::transform3d::{identity4x4, Matrix4x4};
use crate::transform::mul_escalar;

use glam::{Mat4, Vec3, Affine2, Affine3A};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::libc::rand;
use sdl2::rect::{Point, Rect};
use crate::matem;
use crate::parametros::Parametros;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    ancho: f32,
    alto: f32,
}

impl Camera {
    pub fn new(ancho: f32, alto: f32) -> Camera {
        Camera {
            eye: Vec3::new(100.0, 0.0, -150.0),
            center: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            ancho,
            alto,

        }
    }
    pub fn new3(eye: Vec3, center: Vec3, up: Vec3, ancho: f32, alto: f32) -> Camera {
        Camera { eye, center, up, ancho, alto }
    }

    pub fn model_matrix(&self, t: Vec3, s: Vec3, rx: f32, ry: f32, rz: f32) -> Mat4 {
        Mat4::from_translation(t)
            * Mat4::from_scale(s)
            * Mat4::from_rotation_x(rx)
            * Mat4::from_rotation_y(ry)
            * Mat4::from_rotation_z(rz)
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye, self.center, self.up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(
            90.0f32.to_radians(),
            self.ancho / self.alto,
            0.1,
            100.0,
        )
    }

    pub fn dibujar_cubo(canvas: &mut Canvas<Window>, rotation: f32) {
        let anch = canvas.output_size().unwrap().0;
        let alto = canvas.output_size().unwrap().1;

        // Limpiar la pantalla
        canvas.set_draw_color(Color::RGB(0, 100, 0));
        canvas.clear();

        // Definimos las coordenadas de los vértices del cubo
        let vertices: [Vec3; 8] = [
            Vec3::new(-1.0, -1.0, -1.0), // Vértice 0
            Vec3::new(-1.0, -1.0, 1.0), // Vértice 1
            Vec3::new(-1.0, 1.0, -1.0), // Vértice 2
            Vec3::new(-1.0, 1.0, 1.0), // Vértice 3
            Vec3::new(1.0, -1.0, -1.0), // Vértice 4
            Vec3::new(1.0, -1.0, 1.0), // Vértice 5
            Vec3::new(1.0, 1.0, -1.0), // Vértice 6
            Vec3::new(1.0, 1.0, 1.0), // Vértice 7
        ];

        let indices: [(usize, usize, usize); 12] = [
            // Cara frontal
            (0, 1, 2),
            (1, 3, 2),
            // Cara trasera
            (4, 6, 5),
            (5, 6, 7),
            // Cara izquierda
            (0, 2, 4),
            (4, 2, 6),
            // Cara derecha
            (1, 5, 3),
            (5, 7, 3),
            // Cara superior
            (2, 3, 6),
            (3, 7, 6),
            // Cara inferior
            (0, 4, 1),
            (4, 5, 1),
        ];

        let colores: [(u8, u8, u8); 12] = [
            // Cara frontal
            (0, 255, 255),
            (0, 255, 255),
            // Cara trasera
            (255, 0, 255),
            (255, 0, 255),
            // Cara izquierda
            (0, 255, 0),
            (0, 255, 0),
            // Cara derecha
            (255, 255, 0),
            (255, 255, 0),
            // Cara superior
            (0, 0, 255),
            (0, 0, 255),
            // Cara inferior
            (255, 0, 0),
            (255, 0, 0),
        ];

        let p = 200.;  // factor de multiplicación

        // Crear cámara
        let mut camara = Camera::new(anch as f32, alto as f32);

        // Obtener la matriz de total del cubo (puede ser rotada, escalada, etc.)
        let t = Vec3::new(0.0, 0.0, -500.0); // ¿muevo el eje de coordenadas?
        let s = Vec3::new(1.0, 1.0, 1.0);
        let rx = 0.5;
        let ry = rotation;
        let rz = 0.;
        let transform =
            camara.projection_matrix()
                * camara.view_matrix()
                * camara.model_matrix(t, s, rx, ry, rz);

        // Modificamos los vertices segun las matrices y p
        let an = anch as f32 / 2.;
        let al = alto as f32 / 2.;
        let mut v_mod = vec![];

        for i in 0..8 {
            let nuevo = vertices[i] * p;
            let vv = (transform.transform_point3(nuevo) + Vec3::new(an, al, 0.));
            v_mod.push((vv.x as i16, vv.y as i16, vv.z as i16))
        }
        canvas.set_draw_color(Color::RGBA(255, 0, 255, 255));

        // Baricentro y distancia de baricentro a la cámara
        let mut distancias = vec![];
        let mut baricentros = vec![];
        for &(i0, i1, i2) in &indices {
            // Calculo del baricentro
            let b_x = (v_mod[i0].0 as f32 + v_mod[i1].0 as f32 + v_mod[i2].0 as f32) / 3.;
            let b_y = (v_mod[i0].1 as f32 + v_mod[i1].1 as f32 + v_mod[i2].1 as f32) / 3.;
            let b_z = (v_mod[i0].2 as f32 + v_mod[i1].2 as f32 + v_mod[i2].2 as f32) / 3.;

            let b = Vec3::new(b_x, b_y, b_z);

            // Calculo de la distancia de la cámara al baricentro en el eje z
            let d = (b - camara.eye);

            //dbg!(d);
            distancias.push(d.z);

            // Rellenamos vector de baricentros para poder dibujarlos luego
            baricentros.push((b_x as i16, b_y as i16));
        }

        // ordenamos las distancias
        let mut zindices: Vec<usize> = (0..distancias.len()).collect();
        zindices
            .sort_by(|&i, &j| distancias[i]
                .partial_cmp(&distancias[j])
                .unwrap()
                .reverse());

        // Dibuja triangulo relleno
        for w in 0..12 {
            let col = Color::RGB(colores[zindices[w]].0, colores[zindices[w]].1, colores[zindices[w]].2);
            let (i0, i1, i2) = indices[zindices[w]];
            canvas.filled_trigon(v_mod[i0].0, v_mod[i0].1,
                                 v_mod[i1].0, v_mod[i1].1,
                                 v_mod[i2].0, v_mod[i2].1, col);
        }
        //thread::sleep_ms(100);
        // Dibujar cada línea de los triángulos del cubo
        for &(i0, i1, i2) in &indices {
            let st_color = Color::RGB(0, 0, 0);
            // lado 1 del triángulo
            let _ = canvas.thick_line(
                v_mod[i0].0, v_mod[i0].1, v_mod[i1].0, v_mod[i1].1, 2, st_color);
            // lado 2 del triángulo
            let _ = canvas.thick_line(
                v_mod[i0].0, v_mod[i0].1, v_mod[i2].0, v_mod[i2].1, 2, st_color);
            // lado 3 del triángulo
            let _ = canvas.thick_line(
                v_mod[i1].0, v_mod[i1].1, v_mod[i2].0, v_mod[i2].1, 2, st_color);

            // // Calculo del baricentro
            // let b_x = (v_mod[i0].0 as f32 + v_mod[i1].0 as f32 + v_mod[i2].0 as f32) / 3.;
            // let b_y = (v_mod[i0].1 as f32 + v_mod[i1].1 as f32 + v_mod[i2].1 as f32) / 3.;
            // let b_z = (v_mod[i0].2 as f32 + v_mod[i1].2 as f32 + v_mod[i2].2 as f32) / 3.;
            //
            // canvas.filled_circle(b_x as i16, b_y as i16, 5, st_color);
        }

        // Dibujamos baricentros con un circulito creados antes
        let st_color = Color::RGB(0, 0, 0);
        for b in baricentros {
            canvas.filled_circle(b.0, b.1, 5, st_color);
        }
    }
}



