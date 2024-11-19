use image::{DynamicImage, GenericImageView, Rgba};
use crate::color::Color;

pub struct Texture {
    image: DynamicImage,
}

impl Texture {
    pub fn new(file_path: &str) -> Self {
        let image = image::open(file_path).expect("Failed to load texture");
        Texture { image }
    }

    // Devuelve el color de la textura en coordenadas UV
    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let (width, height) = self.image.dimensions();
        let x = (u * width as f32) as u32 % width;
        let y = (v * height as f32) as u32 % height;
        let pixel = self.image.get_pixel(x, y);

        Color::new(pixel[0], pixel[1], pixel[2], pixel[3])  
    }
}