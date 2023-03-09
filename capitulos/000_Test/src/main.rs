use std::env::current_dir;
use std::f32::consts::PI;
use libfinal::environment::size;
use libfinal::image::PImage;
use libfinal::shape::arc;
use sketch::Sketch;

pub mod sketch;

fn main() {
    println!("directorio actual = {:?}", current_dir());
    test02();
}

fn test01() {
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

    game.setup();
    let pimage =
        PImage::load_image(&mut canvas, "resources/imagenes/yuya32.bmp").unwrap();
    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        pimage.update_pixels(&mut canvas);
        //game.draw(&mut canvas);

        canvas.present();
    }
}

fn test02() {
    let mut game = Sketch::new();
    //game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

    game.setup();
    let pimage =
        PImage::load_image(&mut canvas, "resources/imagenes/yuya32.bmp").unwrap();
    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }
        // Angulo 90 grados
        let radianes = 90. * PI / 180.;
        arc(&mut canvas, &mut game.engine.param, 400, 400, 150, 0., radianes);
        //game.draw(&mut canvas);

        canvas.present();
    }
}
