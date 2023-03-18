use libfinal::color::{fill3, stroke3};
use libfinal::getsdl2::*;
use libfinal::image::{load_image_bmp, load_image_png, PImage};
use libfinal::shape::{arc, ellipse};
use sketch::Sketch;
use std::env::current_dir;
use std::f32::consts::PI;
use libfinal::camara::Camera;
use libfinal::environment::{Shader, size, size_opengl};

pub mod sketch;

fn main() {
    println!("directorio actual = {:?}", current_dir());
    let opcion = 7;
    match opcion {
        1 => test01(),
        2 => test02(),
        3 => test03(),
        4 => test04(),
        5 => test05(),
        6 => test06(),
        7 => test07(),

        _ => ()
    }
    test03();
}

fn test01() {
    //let mut zz = 0; // para debug
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut _canvas = size_opengl(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

    game.setup();
    // Carga un vector de 482 elementos, cada elemento tiene un vector de 500 elementos, cada elemento tien 4 bytes
    //let mut pimage = PImage::load_image(&mut canvas, "resources/imagenes/papa32bits.bmp").unwrap();
    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        // Arreglo interno a ventana
        //pimage.update_pixels(&mut canvas); // cuenta tamaño igual a alto
        //game.draw(&mut canvas);
        // Ventana a arreglo interno
        //pimage.load_pixels(&mut canvas); // cuenta tamaño igual a ancho
        // for i in 0..100 {
        //     // BGRA
        //     pimage.image[i] = vec![(0, 0, 255, 0), (0, 0, 255, 0), (0, 0, 255, 0), (0, 0, 255, 0)];
        // }
        //canvas.present();
        //dbg!(zz);
        //zz += 1;
        //dbg!("para breakpoint");
    }
}

fn test02() {
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut _canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

    game.setup();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        // Angulo 90 grados
        //let radianes = 90. * PI / 180.;
        // arc(
        //     &mut canvas,
        //     &mut game.engine.param,
        //     400.,
        //     400.,
        //     150.,
        //     0.,
        //     radianes,
        // );
        // //game.draw(&mut canvas);
        //
        // stroke3(255., 0., 0., &mut game.engine.param);
        // fill3(255., 255., 0., &mut game.engine.param);
        // ellipse(&mut canvas, &mut game.engine.param, 400., 400., 100., 200.);
        //
        // canvas.present();
    }
}

fn test03() {
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);
    let mut imagen = load_image_bmp(&mut canvas, "resources/imagenes/flakes32pba.bmp").unwrap();
    game.setup();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        let x = 15 * 32;
        let y = 15 * 32;
        let img: PImage = imagen.get_trozo(x as usize, y as usize);
        //img.update_pixels(&mut canvas);
        img.image3(&mut canvas, 0, 0, 100, 100);

        //game.draw(&mut canvas);

        canvas.present();
    }
}

fn test04() {
    // let mut game = Sketch::new();
    // //game.pre_load(); // Se ejecuta antes que setup()
    //
    // let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);
    //
    // game.setup();
    // let camera = Camera::new();
    //
    // // Bucle principal ***********************************************************************
    // 'main_loop: loop {
    //     if !game.update() {
    //         break 'main_loop;
    //     }
    //     Camera::dibujar_cubo(&mut canvas, &mut game.engine.param);
    //
    //     //game.draw(&mut canvas);
    //
    //     canvas.present();
    // }
}

fn test05() {
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    //let mut canvas = size(&mut game.engine, 900, 700);
    let ancho = sketch::ANCHO;
    let alto = sketch::ALTO;
    let window = size_opengl(&mut game.engine, ancho, alto);
    //Shader::createshaders(&mut game.engine.param);
    game.setup();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        //Shader::renderiza_opengl(&mut game.engine.param);
        if !game.update() {
            break 'main_loop;
        }
        //Camera::dibujar_cubo(&mut canvas, &mut game.engine.param);

        //game.draw(&mut canvas);

        //Shader::render(&mut game.engine.param);
        Shader::render(&mut game.engine.param);

        // Actualizar una ventana con renderizado OpenGL.
        window.gl_swap_window();

        //canvas.present();
    }
}

fn test06() {
    let ancho = sketch::ANCHO;
    let alto = sketch::ALTO;
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, ancho, alto);

    game.setup();
    let mut rot: f32 = 0.;

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        Camera::dibujar_cubo2(&mut canvas, rot);
        rot += 0.02;

        if rot > 2. * PI {
            rot = 0.;
        }

        game.draw(&mut canvas);

        canvas.present();
    }
}

fn test07() {
    let ancho = sketch::ANCHO;
    let alto = sketch::ALTO;
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, ancho, alto);

    let mut surface = get_surface(100, 100);

    // let start = get_punto(100, 100);
    // let end = get_punto(300, 300);

    cambia_color_surface(&mut surface);

    // let mut canvas2 = surface.into_canvas().unwrap();
    // canvas2.clear();
    // canvas2.set_draw_color(get_color_rgb(0, 0, 255));
    //
    // let mut texture_creator = canvas.texture_creator();
    // let textura = get_texture(800, 800, &mut texture_creator);

    //let texture_creator1 = canvas.texture_creator();
    // let texture1 =
    //     texture_creator1.create_texture_from_surface(&surface).unwrap();

    //let mut texture_creator2 = canvas.texture_creator();
    // let texture2 =
    //     get_texture_de_png(String::from("resources/imagenes/papa.png"), &mut texture_creator2);
    //
    // let mut texture_creator3 = canvas.texture_creator();

    // let texture3 =
    //     get_texture_de_bytes(&mut texture_creator3);

    //let png_file = read_png_file("resources/imagenes/papa.png");
    let pimage0 = load_image_png("resources/imagenes/papa32bits.png").unwrap();

    game.setup();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        //dibuja_textura(&mut canvas, 100, 100, 200, 200, &texture1);
        //dibuja_textura(&mut canvas, 400, 300, 300, 400, &texture2);
        pimage0.update_pixels(&mut canvas);
        //dibuja_textura(&mut canvas, 50, 400, 200, 300, &texture3);
        //game.draw(&mut canvas);
        // canvas2.clear();
        // canvas2.draw_line(start, end);
        // canvas2.fill_rect(get_rect(10, 10, 780, 580));
        //
        // canvas2.copy(&textura, None, None);
        // canvas2.copy(&textura, None, Some(
        //     dame_rect(0, 0, ancho as u32 / 2, alto as u32 / 2)));

        canvas.present();
        //canvas2.present();
    }
}

