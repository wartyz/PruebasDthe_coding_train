/*


    PImage



    PImagen

    Tipo de datos para almacenar imágenes
    crear imagen()

    Crea una nueva PImage (el tipo de datos para almacenar imágenes)

Píxeles

    mezcla()

    Copia un píxel o un rectángulo de píxeles usando diferentes modos de fusión
    Copiar()

    Copia toda la imagen
    filtrar()

    Convierte la imagen a escala de grises o blanco y negro
    cargar píxeles ()

    Carga los datos de píxeles para la ventana de visualización en el de píxeles[] matriz
    mascarilla()

    Enmascara parte de una imagen con otra imagen como un canal alfa
    conseguir()

    Lee el color de cualquier píxel o toma un rectángulo de píxeles
    píxeles[]

    Matriz que contiene los valores de todos los píxeles en la ventana de visualización
    colocar()

    Escribe un color en cualquier píxel o escribe una imagen en otro
    actualizar píxeles ()

    Actualiza la ventana de visualización con los datos en los píxeles[] formación

Cargando y mostrando

    modo de imagen()

    Modifica la ubicación desde la que se dibujan las imágenes.
    imagen()

    Muestra imágenes en la pantalla.
    cargar imagen()

    Carga una imagen en una variable de tipo PImage
    sin tinte ()

    Elimina el valor de relleno actual para mostrar imágenes y vuelve a mostrar imágenes con sus matices originales
    solicitudImagen()

    Carga imágenes en un hilo separado para que su boceto no congelar mientras las imágenes se cargan durante la configuración ()
    tinte()

    Establece el valor de relleno para mostrar imágenes

texturas

    modo textura()

    Establece el espacio de coordenadas para el mapeo de texturas
    envoltura de textura()

    Define si las texturas se repiten o dibujan una vez dentro de un mapa de textura
    textura()

    Establece una textura que se aplicará a los puntos de vértice.



 */
use crate::color::PColor;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum::{ARGB8888, BGRA8888};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::error::Error;
use std::path::Path;
use sdl2::pixels::PixelFormatEnum;
use zimage::GenericImageView;
use crate::matem::pvector3;
use crate::parametros::{Filtros, ImageMode, Parametros};
/* **********************  Image   ****************************************************/
#[derive(Debug, Clone, Default)]
pub struct PImage {
    pub image: Vec<Vec<(u8, u8, u8, u8)>>,
    pub image_width: u32,
    pub image_height: u32,
    tint: (u8, u8, u8),
}

// Ojo-----------Provisional:
pub enum TipoFiltro {
    THRESHOLD,
    GRAY,
    OPAQUE,
    INVERT,
    POSTERIZE,
    BLUR,
    ERODE,
    DILAT,
}

impl PImage {
    // Crea una nueva PImage
    pub fn create_image(x: u32, y: u32) -> PImage {
        let image = vec![vec![(0, 0, 0, 0); y as usize]; x as usize];

        PImage {
            image,
            image_width: x,
            image_height: y,
            tint: (255, 255, 255),
        }
    }

    // Pixels ********************************

    pub fn blend() {
        unimplemented!();
    }

    pub fn copy() {
        unimplemented!();
    }

    pub fn filter(&mut self, filt: Filtros) {
        match filt {
            Filtros::NoFiltro => {}
            Filtros::Gray => self.color_grayscale(),
            _ => {}
        }

        //self.image.color_tint(color);
    }

    /// Carga los datos de píxeles de la ventana en el arreglo self.image.
    /// Siempre se debe llamar a esta función antes de leer o escribir en self.image.
    /// Los cambios posteriores en la ventana de visualización no se reflejarán en píxeles
    /// hasta que se vuelva a llamar a loadPixels().
    pub fn load_pixels(&mut self, canvas: &mut Canvas<Window>) {
        let anch = canvas.viewport().width() as u32;
        let alt = canvas.viewport().height() as u32;

        self.image_width = anch;
        self.image_height = alt;

        let texture_creator = canvas.texture_creator();

        let format = ARGB8888;

        let mut texture = texture_creator
            .create_texture_target(format, anch, alt)
            .unwrap();

        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                //texture_canvas.clear();
                texture_canvas
                    .fill_rect(Rect::new(0, 0, anch, alt))
                    .unwrap();
                // Dibujar lo que quieras en la textura.
            })
            .unwrap();

        // Obtener el formato de píxeles de la textura
        //let format = texture.query().format;

        // Leer los píxeles de la textura en el buffer en orden mal.......
        let bytes = canvas.read_pixels(None, format).unwrap();

        let kk: Vec<Vec<(u8, u8, u8, u8)>> = bytes
            .chunks(4) // crea de [1,2,3,4,5,6..] a [1,2,3,4],[5,6,7,8]...
            // Crea una tupla de cada pequeño arreglo
            .map(|chunk| {
                let b = chunk[0];
                let g = chunk[1];
                let r = chunk[2];
                let a = chunk[3];
                (b, g, r, a)
            })
            .collect::<Vec<(u8, u8, u8, u8)>>()
            // anch es la anchura de la imagen, crea arreglos  de tamaño anchura
            .chunks(anch as usize)
            // convierte cada arreglo a vector
            .map(|row| row.to_vec())
            .collect();

        // Creamos el array transpuesto porque sino no coincide con las demás funciones
        self.image = (0..kk[0].len())
            .map(|col_index| kk.iter().map(|row| row[col_index]).collect())
            .collect();
    }

    pub fn mask() {
        unimplemented!();
    }

    /// aqui devuelvela imagen completa
    pub fn get0(&mut self) {
        unimplemented!();
    }

    /// aqui obtiene el valor de un pixel si está fuera de la ventana se devuelve mnegro
    pub fn get2(&mut self, _x: f32, _y: f32) {
        unimplemented!();
    }

    /// aqui obtiene una seccion del vector en self
    pub fn get4(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, anch: u32, alt: u32) -> Option<PImage> {
        let texture_creator = canvas.texture_creator();

        let format = ARGB8888;
        let texture = texture_creator
            .create_texture_target(format, anch, alt)
            .unwrap();

        let format = texture.query().format;
        dbg!(format); // ARGB8888

        /* canvas
             .with_texture_canvas(&mut texture, |texture_canvas| {
                 texture_canvas
                     .fill_rect(Rect::new(x, y, anch, alt))
                     .unwrap();
                 // Dibujar lo que quieras en la textura.
             })
             .unwrap();*/

        // Obtener el formato de píxeles de la textura
        //let format = texture.query().format;

        let rect = Rect::new(x, y, anch, alt);

        // Leer los píxeles de la textura en el buffer, mal, no en el canvas!!!!!!!!!
        let bytes = canvas.read_pixels(rect, format).unwrap();

        /*   // Creo una surface vacia, no se como enlazarla con self.image
           let surface = Surface::new(anch, alt, BGRA8888).unwrap();

           let pixels = surface.without_lock().unwrap();

           println!(
               "width, height, pitch, size ({:?}, {:?}, {:?}, {:?})",
               surface.width(),
               surface.height(),
               surface.pitch(),
               surface.size()
           );*/
        let kk: Vec<Vec<(u8, u8, u8, u8)>> = bytes
            .chunks(4) // crea de [1,2,3,4,5,6..] a [1,2,3,4],[5,6,7,8]...
            // Crea una tupla de cada pequeño arreglo
            .map(|chunk| {
                let b = chunk[0];
                let g = chunk[1];
                let r = chunk[2];
                let a = chunk[3];
                (b, g, r, a)
            })
            .collect::<Vec<(u8, u8, u8, u8)>>()
            // anch es la anchura de la imagen, crea arreglos  de tamaño anchura
            .chunks(anch as usize)
            // convierte cada arreglo a vector
            .map(|row| row.to_vec())
            .collect();

        // Creamos el array transpuesto porque sino no coincide con las demás funciones
        let image = (0..kk[0].len())
            .map(|col_index| kk.iter().map(|row| row[col_index]).collect())
            .collect();

        Some(PImage {
            image,
            image_width: anch,
            image_height: alt,
            tint: (255, 255, 255),
        })
    }

    // ------pixels[]

    pub fn set() {
        unimplemented!();
    }

    /// Actualiza la ventana de visualización con los datos del arreglo en Self de image.
    pub fn update_pixels(&self, canvas: &mut Canvas<Window>) {
        //dbg!(self.image[100][200]);
        // dbg!(self.image.len());
        // dbg!(self.image[0].len());
        // dbg!(self.image[0][0]);
        //dbg!(self.image[1][0]);

        let texture_creator: TextureCreator<_> = canvas.texture_creator();
        //dbg!(self.image_width); // ancho
        //dbg!(self.image_height); // alto

        let format = ARGB8888;
        //let format = BGRA8888; // prueba cambiando el formato
        let mut texture: Texture = texture_creator
            .create_texture_streaming(format, self.image_width, self.image_height)
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], _: usize| {
                //dbg!(self.image.len()); //500
                //for y in 0..self.image.len() {
                for x in 0..self.image_width as usize {
                    //dbg!(self.image[y].len());
                    //for x in 0..self.image[y].len() {
                    for y in 0..self.image[x].len() {
                        //for x in 0..self.image_height as usize {
                        // dbg!(x);
                        // dbg!(y);
                        let index = (y * self.image_width as usize + x) * 4;
                        //let index = y * 4;
                        //dbg!(index);
                        //dbg!(self.image[y][x].0);
                        /*buffer[index] = self.image[y][x].0;
                        buffer[index + 1] = self.image[y][x].1;
                        buffer[index + 2] = self.image[y][x].2;
                        buffer[index + 3] = self.image[y][x].3;*/
                        //dbg!(y);
                        //dbg!(x);
                        //dbg!(index);
                        /* buffer[index] = self.image[y][x].0; // B
                        buffer[index + 1] = self.image[y][x].1; // G
                        buffer[index + 2] = self.image[y][x].2; // R
                        buffer[index + 3] = self.image[y][x].3; // A*/

                        buffer[index] = self.image[x][y].0; // B
                        buffer[index + 1] = self.image[x][y].1; // G
                        buffer[index + 2] = self.image[x][y].2; // R
                        buffer[index + 3] = self.image[x][y].3; // A
                    }
                }
            })
            .expect("Error en update_pixels");

        // Renderiza la textura en la ventana
        canvas.set_blend_mode(BlendMode::Blend);

        let dst_rect = Rect::new(0, 0, self.image_width, self.image_height);
        //dbg!(texture.query().height); // 482
        //dbg!(texture.query().width); // 500
        //dbg!(texture.query().format); // RGB888
        //dbg!(texture.query().access); // Streaming
        canvas.copy(&texture, None, dst_rect).unwrap();
    }

    // Loading & Displaying ********************************

    pub fn image_mode(param: &mut Parametros, modo: ImageMode) {
        param.image_mode = modo;
    }

    pub fn image3(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, destx: u32, desty: u32) {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        let format = ARGB8888;

        let mut texture: Texture = texture_creator
            .create_texture_streaming(format, self.image_width, self.image_height)
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], _: usize| {
                for x in 0..self.image_width as usize {
                    for y in 0..self.image[x].len() {
                        let index = (y * self.image_width as usize + x) * 4;

                        buffer[index] = self.image[x][y].0; // B
                        buffer[index + 1] = self.image[x][y].1; // G
                        buffer[index + 2] = self.image[x][y].2; // R
                        buffer[index + 3] = self.image[x][y].3; // A
                    }
                }
            })
            .expect("Error en image3");

        // Renderiza la textura en la ventana
        canvas.set_blend_mode(BlendMode::Blend);
        dbg!(x, y);
        //let dst_rect = Rect::new(x, y, self.image_width, self.image_height);
        let dst_rect = Rect::new(x, y, destx, desty);

        canvas.copy(&texture, None, dst_rect).unwrap();
    }

    pub fn image5(&self, canvas: &mut Canvas<Window>, param: &mut Parametros, x_vieja: i32, y_vieja: i32, ancho: u32, alto: u32) {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        let format = ARGB8888;

        let mut texture: Texture = texture_creator
            .create_texture_streaming(format, ancho as u32, alto as u32)
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], _: usize| {
                for x in 0..ancho as usize {
                    for y in 0..alto as usize {
                        let index = (y * ancho as usize + x) * 4;

                        buffer[index] = self.image[x][y].0; // B
                        buffer[index + 1] = self.image[x][y].1; // G
                        buffer[index + 2] = self.image[x][y].2; // R
                        buffer[index + 3] = self.image[x][y].3; // A
                    }
                }
            })
            .expect("Error en image3");

        // Renderiza la textura en la ventana
        //canvas.set_blend_mode(BlendMode::Blend);
        //dbg!(param.matriz_total);
        let p = param.matriz_total * pvector3(x_vieja as f32, y_vieja as f32, 1.0);
        //dbg!(x_vieja, y_vieja);
        //dbg!(p.x,p.y);
        //let dst_rect = Rect::new(x, y, self.image_width, self.image_height);
        //dbg!(ancho);
        //dbg!(alto);

        let dst_rect = Rect::new(p.x as i32, p.y as i32, ancho, alto);

        canvas.copy(&texture, None, dst_rect).unwrap();
    }

    pub fn no_tint() {
        unimplemented!();
    }

    pub fn request_image() {
        unimplemented!();
    }

    pub fn tint() {
        unimplemented!();
    }

    // Textures ********************************

    pub fn texture_mode() {
        unimplemented!();
    }

    pub fn texture_wrap() {
        unimplemented!();
    }

    pub fn texture() {
        unimplemented!();
    }

    // Función inventada por mi
    pub fn get_trozo(&self, x0: usize, y0: usize) -> PImage {
        //dbg!(x0,y0);

        let mut trozo: Vec<Vec<(u8, u8, u8, u8)>> = vec![];
        for x in 0..32 {
            let mut colum: Vec<(u8, u8, u8, u8)> = vec![];
            for y in 0..32 {
                //colum.push(self.image[x + x0][y + y0].clone())
                colum.push(self.image[x + x0][y + y0].clone())
            }
            trozo.push(colum);
        }

        PImage {
            image: trozo,
            image_width: 32,
            image_height: 32,
            tint: (255, 255, 255),
        }
    }

    // Función creada por mi, convierte self.image a grises

    fn color_grayscale(&mut self) {
        let mut result = Vec::with_capacity(self.image.len());
        for row in &self.image {
            let mut new_row = Vec::with_capacity(row.len());
            for pixel in row {
                let gray_value = (pixel.0 as f32 * 0.3 + pixel.1 as f32 * 0.59 + pixel.2 as f32 * 0.11) as u8;
                let gray_pixel = (gray_value, gray_value, gray_value, pixel.3);
                new_row.push(gray_pixel);
            }
            result.push(new_row);
        }
        self.image = result;
    }
}

// ****** Funciones creadas por mi ************************************
// Recibe un PColor y devuelve un vector Vec<(u8, u8, u8, u8)>
pub fn pcolor_to_vector(pcolor: PColor) -> Vec<(u8, u8, u8, u8)> {
    vec![pcolor_to_tupla(pcolor)]
}

// Recibe un PColor y devuelve una tupla (u8, u8, u8, u8)
pub fn pcolor_to_tupla(pcolor: PColor) -> (u8, u8, u8, u8) {
    (pcolor.r, pcolor.g, pcolor.b, pcolor.a)
}

pub fn pimage_vacio() -> PImage {
    PImage {
        image: vec![],
        image_width: 0,
        image_height: 0,
        tint: (255, 255, 255),
    }
}

// pub fn lee_png(canvas: &mut Canvas<Window>) -> Texture<'static> {
//
// }

pub fn renderiza(canvas: &mut Canvas<Window>, param: &mut Parametros, x_vieja: f32, y_vieja: f32, ancho: u32, alto: u32) {
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let texture: Texture = texture_creator.load_texture("resources/imagenes/flakes32.png").unwrap();

    //canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
    //canvas.clear();
    let p = param.matriz_total * pvector3(x_vieja as f32, y_vieja as f32, 1.0);

    //let TextureQuery { width, height, .. } = texture.query();
    let sce = Rect::new(0, 0, 32, 32);
    let dst = Rect::new(p.x as i32, p.y as i32, ancho, alto);

    canvas.copy(&texture, Some(sce), Some(dst));
}

// Carga una imagen en una variable de tipo PImage.
// Solo carga .bmp   debe ser de formato ARGB8888 osea 32 bits
pub fn load_image_bmp(
    canvas: &mut Canvas<Window>,
    filename: &str,
) -> Option<PImage> {
    let texture_creator = canvas.texture_creator();
    let path = Path::new(filename);
    dbg!(path);

    let texture = texture_creator.load_texture(path).unwrap();
    let format = texture.query().format;
    dbg!(format); // ARGB8888

    let image_width = texture.query().width;
    let image_height = texture.query().height;

    let surface = Surface::load_bmp(filename).unwrap();

    let pixels = surface.without_lock().unwrap();

    println!("Path {:?}", Path::new(filename));
    println!(
        "width, height, pitch, size ({:?}, {:?}, {:?}, {:?})",
        surface.width(),
        surface.height(),
        surface.pitch(),
        surface.size()
    );

    // Creamos un vector para contener los datos de la imagen
    let mut image: Vec<Vec<(u8, u8, u8, u8)>> = vec![];
    /* for y in 0..image_height as usize {
    let mut vv = vec![];
    for x in 0..image_width as usize {
        let index = (y * image_width as usize + x) * 4;*/

    for x in 0..image_width as usize {
        let mut vv = vec![];
        for y in 0..image_height as usize {
            let index = (y * image_width as usize + x) * 4;

            let valor = (
                pixels[index],
                pixels[index + 1],
                pixels[index + 2],
                pixels[index + 3],
            );

            vv.push(valor);
        }

        image.push(vv);
    }

    Some(PImage {
        image,
        image_width,
        image_height,
        tint: (255, 255, 255),
    })
}

// Desde un fichero png, crea PImage
pub fn load_image_png(filename: &str) -> Option<PImage> {
    let mut p = pimage_vacio();
    let format = zimage::guess_format(filename.as_bytes());
    dbg!(format);
    if let Ok(pimage) = zimage::open(filename) { // ImageReader::open(filename) {
        //let (width, height) = pimage.into_dimensions().unwrap();

        let (width, height) = pimage.dimensions();
        //let width = 500;
        //let height = 482;

        let mut pixels = Vec::with_capacity((width * height) as usize);

        //for y in 0..height {
        for x in 0..width {
            //let mut row = Vec::with_capacity(width as usize);
            let mut row = Vec::with_capacity(height as usize);
            //for x in 0..width {
            for y in 0..height {
                let pixel = pimage.get_pixel(x, y);
                // Es RGBA y hay que pasarlo a BGRA para que lo puedan leer las otras funciones
                row.push((pixel[2], pixel[1], pixel[0], pixel[3]));
            }
            pixels.push(row);
        }
        dbg!(pixels.len());
        dbg!(pixels[0].len());
        dbg!(pixels[0][0]);
        dbg!(pixels[1][0]);
        p.image = pixels;
        dbg!(width,height);
        p.image_width = width;
        p.image_height = height;

        Some(p)
    } else {
        None
    }
}


