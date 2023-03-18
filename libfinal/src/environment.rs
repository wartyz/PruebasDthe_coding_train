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

use std::ffi::CString;
use std::{str, mem, ptr};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::engine::Engine;

use gl::types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr, GLsizei, GLvoid, GLboolean};
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum::ABGR8888;
use sdl2::rect::{Point, Rect};
use sdl2::surface;
use sdl2::surface::Surface;

use crate::parametros::Parametros;
use crate::render_gl;

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

    // Inicializa sdl2 devuelve Result<Sdl, String>
    let sdl_context = sdl2::init().expect("¡Error al crear el contexto SDL2!");

    // video devuelve Result<VideoSubsystem, String>
    let video_subsystem = sdl_context.video().expect("¡Error al obtener el subsistema de video SDL2!");

    let window = video_subsystem // Se ha creado constructor de Window
        .window(titulo, ancho, alto) // título, ancho, alto y devuelve un WindowBuilder
        .position_centered()
        .opengl()
        .build()// Construye Window Devuelve un Result<Window, WindowBuildError>
        .map_err(|e| e.to_string())
        .unwrap();// Maneja el posible error

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    //let _event_pump = sdl_context.event_pump().expect("¡Error al obtener la bomba de eventos SDL2!");

    // let _surface = canvas.window().surface(&event_pump);

    //let _texture_creator = canvas.texture_creator();

    let c = Color::RGB(
        engine.param.color_background.r,
        engine.param.color_background.g,
        engine.param.color_background.b,
    );

    canvas.set_draw_color(c);
    canvas.clear();

    println!("rendering 4");

    engine.sdl_context = Some(sdl_context);

    canvas
}

pub fn size_opengl(engine: &mut Engine, ancho: u32, alto: u32) -> Window {
    // Inicializamos ventana
    engine.param.ancho = ancho as f32;
    engine.param.alto = alto as f32;

    let titulo = "Ventanita";

    // Inicializa sdl2 devuelve Result<Sdl, String>
    let sdl_context = sdl2::init().expect("¡Error al crear el contexto SDL2!");

    // video devuelve Result<VideoSubsystem, String>
    let video_subsystem = sdl_context.video().expect("¡Error al obtener el subsistema de video SDL2!");

    // gl_attr devuelve una estructura GLAttr
    let gl_attr = video_subsystem.gl_attr();

    // set_context_profile establece el Core profile
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5); // Establece version 4.5

    let window = video_subsystem // Se ha creado constructor de Window
        .window(titulo, ancho, alto) // título, ancho, alto y devuelve un WindowBuilder
        .position_centered()
        .opengl()
        .build()// Construye Window Devuelve un Result<Window, WindowBuildError>
        .map_err(|e| e.to_string())
        .unwrap();// Maneja el posible error

    // gl_create_context devuelve Result<GLContext, String>
    let _gl_context = window.gl_create_context().unwrap();

    // Una función para cargar el puntero de una función OpenGL con un string
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    Shader::createshaders(&mut engine.param);

    //let _event_pump = sdl_context.event_pump().expect("¡Error al obtener la bomba de eventos SDL2!");

    println!("rendering 5");

    engine.sdl_context = Some(sdl_context);

    window
}

pub fn smooth() { unimplemented!(); }

pub fn width() { unimplemented!(); }

// ***** Funciones creadas por mi ***********************************************************************

pub struct Shader {}

impl Shader {
    pub fn createshaders(param: &mut Parametros) {
        // Compilar Vertex Shader y Fragment Shader.
        // La struct std::ffi::CString es un tipo compatible con C,
        // terminada en nulo, sin nulos en el medio

        // La macro include_str! incrusta el contenido de un archivo UTF-8 como un valor estático &str.
        let vert_shader = render_gl::Shader::from_vert_source(
            &CString::new(
                include_str!(
                    "h:/Proyectos_rust/pruebasD_the_coding_train_SDL2/libfinal/resources/shaders/triangle.vert"))
                .unwrap(),
        ).unwrap();

        let frag_shader = render_gl::Shader::from_frag_source(
            &CString::new(
                include_str!(
                    "h:/Proyectos_rust/pruebasD_the_coding_train_SDL2/libfinal/resources/shaders/triangle.frag"))
                .unwrap(),
        ).unwrap();
        // Crea programa shader enlazando VS y FS
        param.shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        // Creamos el objeto de buffer de vértices ******************* Triángulo
        let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        // Creamos un VBO vacio
        param.vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut param.vbo);
        }
        // Ahora cargamos datos en el buffer
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, param.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // objetivo
                // Tamaño de los datos en bytes tomando el tamaño de un f32 con std::mem::size_of::<f32>(),
                // multiplicándolo por el número de elementos en el vector (vertices.len()),
                // y luego forzándolo al entero gl::types::GLsizeiptr
                (vertices.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                // Obtenemos un puntero vacío a los datos mediante la función .as_ptr(),
                // pero nos devuelve *const f32, mientras que OpenGL necesita *const GLvoid
                vertices.as_ptr() as *const gl::types::GLvoid, // puntero a datos
                gl::STATIC_DRAW,                               // uso
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // desenlazar el buffer
        }
        // Creamos el VAO
        param.vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut param.vao);
        }

        unsafe {
            gl::BindVertexArray(param.vao); // Vinculamos el VAO
            gl::BindBuffer(gl::ARRAY_BUFFER, param.vbo); // Vinculamos el VBO
            gl::EnableVertexAttribArray(0); // esto es "layout (location = 0)" en el vertex shader
            gl::VertexAttribPointer(
                0,         // índice del atributo de vértices genérico ("layout (location = 0)")
                3,         // número de componentes del atributo de vértices genérico
                gl::FLOAT, // tipo de datos
                gl::FALSE, // normalizado (conversión int-a-float )
                // stride (separación en bytes entre attributos consecutivos)
                (3 * mem::size_of::<f32>()) as gl::types::GLint,
                ptr::null(), // desplazamiento del primer componente
            );
            // desvinculamos VBO y VAO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        // set up shared state for window

        unsafe {
            gl::Viewport(0, 0, 900, 700); // configurar el viewport
            gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Color de limpieza
        }
    }

    pub fn render(param: &mut Parametros) {
        // renderizar contenidos de ventana aquí
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // Dibuja triangulo
        // Da la orden de uso de programa shader
        param.shader_program.set_used();
        unsafe {
            gl::BindVertexArray(param.vao);
            gl::DrawArrays(
                gl::TRIANGLES, // modo
                0,             // índice inicial en el arreglo habilitado
                3,             // número de índices a renderizar
            );
        }

        // Actualizar una ventana con renderizado OpenGL.
        //window.gl_swap_window();
    }
}