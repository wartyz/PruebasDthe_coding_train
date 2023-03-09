#[cfg(test)]
pub mod tests {
    use std::env::current_dir;

    //use crate::{sketch, Sketch};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn pba01() {
        /* //dbg!(current_dir());
         let mut game = Sketch::new();
         game.pre_load(); // Se ejecuta antes que setup()
         let mut canvas = createcanvas(&mut game.engine, sketch::ANCHO as u32, sketch::ALTO as u32);

         // let texture_creator = canvas.texture_creator();
         // //
         // let texture = texture_creator
         //     .load_texture("../../resources/imagenes/yuya32.bmp")
         //     .unwrap();

         let pimage =
             PImage::load_image(&mut canvas, "../../resources/imagenes/yuya32.bmp").unwrap();
         dbg!(&pimage);
         //for _ in 0..50 {

         //}
         'main_loop: loop {
             pimage.update_pixels(&mut canvas);
             if !game.update() {
                 break 'main_loop;
             }
             canvas.present();
         }*/
    }
}
