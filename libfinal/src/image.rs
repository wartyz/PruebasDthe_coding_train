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
use sdl2::rect::Rect;
use sdl2::render::{Canvas, WindowCanvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use crate::color::PColor;
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
    pub fn create_image() { unimplemented!(); }

// Pixels ********************************

    pub fn blend() { unimplemented!(); }

    pub fn copy() { unimplemented!(); }

    pub fn filter() { unimplemented!(); }

    // Carga los datos de píxeles de la ventana en el arreglo self.image.
    // Siempre se debe llamar a esta función antes de leer o escribir en self.image.
    // Los cambios posteriores en la ventana de visualización no se reflejarán en píxeles
    // hasta que se vuelva a llamar a loadPixels().
    pub fn load_pixels(&mut self, canvas: &mut WindowCanvas) {
        //}  {

        let anch = canvas.viewport().width() as u32;
        let alt = canvas.viewport().height() as u32;

        dbg!(anch);
        dbg!(alt);
        self.image_width = anch;
        self.image_height = alt;

        let texture_creator = canvas.texture_creator();

        dbg!(canvas.viewport().width());
        let mut texture = texture_creator
            .create_texture_target(texture_creator.default_pixel_format(), anch, alt)
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
        let format = texture.query().format;

        // Leer los píxeles de la textura en el buffer
        let bytes = canvas.read_pixels(None, format).unwrap();

        // Rellenamos la imagen en Self
        self.image = bytes
            .chunks(4)
            .map(|chunk| {
                let r = chunk[0];
                let g = chunk[1];
                let b = chunk[2];
                let a = chunk[3];
                (r, g, b, a)
            })
            .collect::<Vec<(u8, u8, u8, u8)>>()
            .chunks(alt as usize)
            .map(|row| row.to_vec())
            .collect();

        dbg!(self.image.len());
    }

    pub fn mask() { unimplemented!(); }

    pub fn get() { unimplemented!(); }

// ------pixels[]

    pub fn set() { unimplemented!(); }

    // Actualiza la ventana de visualización con los datos del arreglo en Self de image.
    // Úselo junto con loadPixels().  Si solo está leyendo píxeles del vector,
    // no es necesario llamar a updatePixels(); la actualización solo es necesaria para aplicar los cambios.
    pub fn update_pixels(&self, canvas: &mut Canvas<Window>) {
        // -> Result<(), Box<dyn Error>> {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();
        let mut texture: Texture = texture_creator
            .create_texture_streaming(None, self.image_width, self.image_height)
            .unwrap();

        // Lock texture for writing
        let mut texture_buffer: Vec<u8> = vec![0; 640 * 480 * 4];
        texture
            .with_lock(None, |buffer: &mut [u8], _: usize| {
                //dbg!(self.image.len());
                for y in 0..self.image.len() {
                    //for y in 0..self.image_width as usize {
                    //dbg!(self.image[y].len());
                    for x in 0..self.image[y].len() {
                        //for x in 0..self.image_height as usize {
                        //dbg!(x);
                        //dbg!(y);
                        let index = (y * self.image_width as usize + x) * 4;
                        //let index = y * 4;
                        //dbg!(index);
                        buffer[index] = self.image[y][x].0;
                        buffer[index + 1] = self.image[y][x].1;
                        buffer[index + 2] = self.image[y][x].2;
                        buffer[index + 3] = self.image[y][x].3;
                    }
                }
            })
            .expect("Error en update_pixels");
    }

// Loading & Displaying ********************************

    pub fn image_mode() { unimplemented!(); }

    pub fn image() { unimplemented!(); }

    pub fn load_image() { unimplemented!(); }

    pub fn no_tint() { unimplemented!(); }

    pub fn request_image() { unimplemented!(); }

    pub fn tint() { unimplemented!(); }

// Textures ********************************

    pub fn texture_mode() { unimplemented!(); }

    pub fn texture_wrap() { unimplemented!(); }

    pub fn texture() { unimplemented!(); }
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

