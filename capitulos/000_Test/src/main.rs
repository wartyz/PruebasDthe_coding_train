use libfinal::color::{fill3, stroke3};
use libfinal::environment::size;
use libfinal::image::PImage;
use libfinal::shape::{arc, ellipse};
use sketch::Sketch;
use std::env::current_dir;
use std::f32::consts::PI;

pub mod sketch;

fn main() {
    println!("directorio actual = {:?}", current_dir());
    test01();
}

fn test01() {
    //let mut zz = 0; // para debug
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

    game.setup();
    // Carga un vector de 482 elementos, cada elemento tiene un vector de 500 elementos, cada elemento tien 4 bytes
    let mut pimage = PImage::load_image(&mut canvas, "resources/imagenes/papa32bits.bmp").unwrap();
    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        // Arreglo interno a ventana
        pimage.update_pixels(&mut canvas); // cuenta tamaño igual a alto
        //game.draw(&mut canvas);
        // Ventana a arreglo interno
        pimage.load_pixels(&mut canvas); // cuenta tamaño igual a ancho
        // for i in 0..100 {
        //     // BGRA
        //     pimage.image[i] = vec![(0, 0, 255, 0), (0, 0, 255, 0), (0, 0, 255, 0), (0, 0, 255, 0)];
        // }
        canvas.present();
        //dbg!(zz);
        //zz += 1;
        //dbg!("para breakpoint");
    }
}

fn test02() {
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

    game.setup();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        // Angulo 90 grados
        let radianes = 90. * PI / 180.;
        arc(
            &mut canvas,
            &mut game.engine.param,
            400.,
            400.,
            150.,
            0.,
            radianes,
        );
        //game.draw(&mut canvas);

        stroke3(255., 0., 0., &mut game.engine.param);
        fill3(255., 255., 0., &mut game.engine.param);
        ellipse(&mut canvas, &mut game.engine.param, 400., 400., 100., 200.);

        canvas.present();
    }
}
