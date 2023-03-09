use std::env::current_dir;
use libfinal::environment::size;
use libfinal::image::PImage;
use sketch::Sketch;

pub mod sketch;

fn main() {
    println!("directorio actual = {:?}", current_dir());

    let mut game = Sketch::new();
    game.pre_load(); // Se ejecuta antes que setup()

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
        game.draw(&mut canvas);

        canvas.present();
    }
}
