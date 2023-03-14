use std::env::current_dir;
use libfinal::environment::size;
use libfinal::image::PImage;
use sketch::Sketch;

pub mod sketch;
pub mod snowflake;

fn main() {
    println!("directorio actual = {:?}", current_dir());

    let mut game = Sketch::new();
    game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);
    let mut imagen = PImage::load_image(&mut canvas, "resources/imagenes/flakes32.bmp").unwrap();
    //let texture_creator: TextureCreator<_> = canvas.texture_creator();
    //texture_creator.load_texture("resources/imagenes/flakes32.png").unwrap();
    game.setup(&mut imagen); // , &mut imagen);

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }

        game.draw(&mut canvas);

        canvas.present();
    }
}
