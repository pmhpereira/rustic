use super::Texture;

use nalgebra::Vector3;

use image::{io::Reader as ImageReader, ImageBuffer, Pixel, Rgb};

pub struct ImageTexture {
    width: u32,
    height: u32,
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl ImageTexture {
    pub fn new(file_path: String) -> ImageTexture {
        let image = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = image.width();
        let height = image.height();
        let data = image.as_rgb8().unwrap();

        ImageTexture {
            width,
            height,
            data: data.clone(),
        }
    }
}

impl Texture for ImageTexture {
    fn get_color(&self, (u, v): (f64, f64), _point: &Vector3<f64>) -> Vector3<f64> {
        let x = (u * self.width as f64) as u32;
        let y = ((1.0 - v) * self.height as f64) as u32;

        let pixel = self.data.get_pixel(x, y).to_rgb();
        let channels = pixel.channels();

        Vector3::new(
            channels[0] as f64 / 255.0,
            channels[1] as f64 / 255.0,
            channels[2] as f64 / 255.0,
        )
    }
}
