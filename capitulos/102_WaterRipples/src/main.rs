use std::env::current_dir;
use libfinal::environment::size;
use sketch::Sketch;

pub mod sketch;

fn main() {
    println!("directorio actual = {:?}", current_dir());
    // Command::new("ls")
    //     //.current_dir("/bin")
    //     .spawn()
    //     .expect("ls command failed to start");

    let mut game = Sketch::new();
    game.pre_load(); // Se ejecuta antes que setup()

    let mut canvas = size(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

    game.setup();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }

        game.draw(&mut canvas);
        game.mouse_dragged();

        canvas.present();
    }
}
