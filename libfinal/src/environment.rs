/*
    cursor()
    delay()
    displayDensity()
    displayHeight
    displayWidth
    focused
    frameCount
    frameRate
    frameRate()
    height
    fullScreen()
    noCursor()
    noSmooth()
    pixelDensity()
    pixelHeight
    pixelWidth
    settings()
    size()
    smooth()
    width
 */

use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::engine::Engine;

pub fn cursor() { unimplemented!(); }

pub fn delay() { unimplemented!(); }

pub fn display_density() { unimplemented!(); }

pub fn display_height() { unimplemented!(); }

pub fn display_width() { unimplemented!(); }

pub fn focused() { unimplemented!(); }

pub fn frame_count() { unimplemented!(); }

// variable frame_rate

pub fn frame_rate() { unimplemented!(); }

pub fn height() { unimplemented!(); }

pub fn full_screen(engine: &mut Engine) {
    engine.param.full_screen = true;
}

pub fn no_cursor() { unimplemented!(); }

pub fn no_smooth() { unimplemented!(); }

pub fn pixel_density() { unimplemented!(); }

pub fn pixel_height() { unimplemented!(); }

pub fn pixel_width() { unimplemented!(); }

pub fn settings() { unimplemented!(); }

pub fn size(engine: &mut Engine, ancho: u32, alto: u32) -> Canvas<Window> {
    // Inicializamos ventana
    engine.param.ancho = ancho as f32;
    engine.param.alto = alto as f32;

    let titulo = "Ventanita";

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(titulo, ancho, alto)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string());

    let mut canvas = window
        .unwrap()
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let event_pump = sdl_context.event_pump().unwrap();
    let _surface = canvas.window().surface(&event_pump);
    let _texture_creator = canvas.texture_creator();

    let c = sdl2::pixels::Color::RGB(
        engine.param.color_background.r,
        engine.param.color_background.g,
        engine.param.color_background.b,
    );

    canvas.set_draw_color(c);
    canvas.clear();

    //canvas.as_mut().unwrap().set_draw_color(c);

    //engine.canvas.as_mut().unwrap().clear();
    //canvas.as_mut().unwrap().present();
    println!("rendering 4");

    engine.sdl_context = Some(sdl_context);

    canvas
}

pub fn smooth() { unimplemented!(); }

pub fn width() { unimplemented!(); }