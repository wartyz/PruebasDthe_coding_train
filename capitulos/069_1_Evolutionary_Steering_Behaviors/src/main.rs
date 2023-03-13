// https://www.youtube.com/watch?v=XaOVH8ZSRNA&list=PLRqwX-V7Uu6ZiZxtDDRCi6uhfTH4FilpH&index=96
// Video 2   acabado

use libfinal::environment::size;

use sketch::Sketch;

pub mod vehicle;

pub mod sketch;

fn main() {
    println!("Hola, world!");

    let mut game = Sketch::new();

    let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

    game.setup();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }

        game.draw(&mut canvas);

        //game.engine.canvas.as_mut().unwrap().clear();
        //game.engine.canvas.as_mut().unwrap().present();
        game.mouse_dragged();
        canvas.present();
    }
}
