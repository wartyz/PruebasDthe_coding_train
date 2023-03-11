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
    pub fn create_image() {
        unimplemented!();
    }

    // Pixels ********************************

    pub fn blend() {
        unimplemented!();
    }

    pub fn copy() {
        unimplemented!();
    }

    pub fn filter() {
        unimplemented!();
    }

    // Carga los datos de píxeles de la ventana en el arreglo self.image.
    // Siempre se debe llamar a esta función antes de leer o escribir en self.image.
    // Los cambios posteriores en la ventana de visualización no se reflejarán en píxeles
    // hasta que se vuelva a llamar a loadPixels().
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

    pub fn get() {
        unimplemented!();
    }

    // ------pixels[]

    pub fn set() {
        unimplemented!();
    }

    // Actualiza la ventana de visualización con los datos del arreglo en Self de image.

    pub fn update_pixels(&self, canvas: &mut Canvas<Window>) {
        //dbg!(self.image[100][200]);
        //dbg!(self.image.len()); // 482
        //dbg!(self.image[0][0]);
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

    pub fn image_mode() {
        unimplemented!();
    }

    pub fn image() {
        unimplemented!();
    }

    // Carga una imagen en una variable de tipo PImage.
    // Solo carga .bmp   debe ser de formato ARGB8888 osea 32 bits
    pub fn load_image(
        canvas: &mut Canvas<Window>,
        filename: &str,
    ) -> Result<PImage, Box<dyn Error>> {
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

        Ok(PImage {
            image,
            image_width,
            image_height,
            tint: (255, 255, 255),
        })
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
